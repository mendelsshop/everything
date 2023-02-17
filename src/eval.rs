use log::info;

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    mem::swap,
    rc::Rc,
};

use crate::{
    error::{arg_error, error},
    parser::{
        rules::{IdentifierType, LiteralType, OtherStuff, Stuff},
        Thing,
    },
    token::TokenType,
};

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    Ok(contents.clone())
}

pub fn write_file(
    file_name: &str,
    contents: &str,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match mode {
        "w" => {
            // overwrite existing contents
            // clear file contents
            let mut file = OpenOptions::new().write(true).open(file_name)?;
            file.set_len(0)?;
            Ok(file.write_all(contents.as_bytes())?)
        }
        "a" => {
            let mut file = OpenOptions::new().append(true).open(file_name)?;
            Ok(file.write_all(contents.as_bytes())?)
        }
        _ => Err("Invalid mode")?,
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub inside: LiteralOrFile,
    pub print: bool,
    pub line: i32,
    pub new_line: bool,
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.print {
                if self.new_line {
                    format!("{}\n", self.inside)
                } else {
                    format!("{}", self.inside)
                }
            } else {
                String::new()
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LitOrList {
    Identifier(Box<NewList>),
    Literal(LiteralOrFile),
}

impl Display for LitOrList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(i) => write!(f, "{i}"),
            Self::Literal(l) => write!(f, "{l}"),
        }
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct NewList {
    pub car: LitOrList,
    pub cdr: LitOrList,
}

impl NewList {
    pub fn new(mut thing: Vec<LiteralOrFile>) -> Self {
        Self {
            car: LitOrList::Literal(thing.remove(0)),
            cdr: LitOrList::Literal(thing.remove(0)),
        }
    }
}

impl Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.car, self.cdr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralOrFile {
    Literal(LiteralType),
    File(String),
}

impl LiteralOrFile {
    pub fn get_file(self, line: i32, keyword: &TokenType) -> String {
        match self {
            Self::File(file) => file,
            _ => {
                error(line, format!("{keyword} requires a file"));
            }
        }
    }
    pub fn get_string(self, line: i32, keyword: &TokenType) -> String {
        match self {
            Self::Literal(LiteralType::String(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a string"));
            }
        }
    }
    pub fn get_number(self, line: i32, keyword: &TokenType) -> f64 {
        match self {
            Self::Literal(LiteralType::Number(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a number"));
            }
        }
    }
    pub fn get_bool(self, line: i32, keyword: &TokenType) -> bool {
        match self {
            Self::Literal(LiteralType::Boolean(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a boolean"));
            }
        }
    }
}

impl Display for LiteralOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::File(file) => write!(f, "{file}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NewVairable {
    pub value: LiteralOrFile,
}

impl NewVairable {
    pub const fn new(value: LiteralOrFile) -> Self {
        Self { value }
    }
}

impl Display for NewVairable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum NewIdentifierType {
    List(Box<NewList>),
    Vairable(Box<NewVairable>),
}

impl NewIdentifierType {
    pub fn to_vec_literaltype(self, line: i32) -> Vec<LiteralOrFile> {
        match self {
            Self::Vairable(v) => vec![v.value],
            _ => error(line, "cannot convert to vec"),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub vars: HashMap<String, NewIdentifierType>,
    pub function: HashMap<char, (Vec<Thing>, f64)>,
    pub parent_scope: Option<Box<Scope>>,
    pub files: HashMap<String, File>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: None,
            files: HashMap::new(),
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: Some(parent),
            files: HashMap::new(),
        }
    }
    pub fn set_var(
        &mut self,
        name: &str,
        value: &mut Vec<LiteralOrFile>,
        recurse: bool,
        line: i32,
    ) {
        // the reason for this being its own method vs using the set method is because it will be easier to use/implemnet getting variable from different scopes
        // and also less typing instead of creating a NewIdentifierType you just pass in a vector of LiteralType
        let new_val: NewIdentifierType = match value.len() {
            0 => error(line, "expected Identifier, got empty list"),
            1 => NewIdentifierType::Vairable(Box::new(NewVairable::new(value.clone().remove(0)))),
            2 => NewIdentifierType::List(Box::new(NewList::new(value.clone()))),
            _ => error(
                line,
                "expected Identifier, got list with more than 2 elements",
            ),
        };
        match name {
            name if name.ends_with(".car") | name.ends_with(".cdr") => {
                let new_name = name.trim_end_matches(".car").trim_end_matches(".cdr");
                if recurse {
                    if self.has_var(new_name, false) {
                        let mut new_var = match self.get_var(new_name, line) {
                            NewIdentifierType::List(list) => list,
                            _ => error(line, "expected list"),
                        };
                        if name.ends_with(".cdr") {
                            new_var.cdr = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                            };
                        } else {
                            new_var.car = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                            };
                        }
                        self.vars
                            .insert(new_name.to_string(), NewIdentifierType::List(new_var));
                    } else {
                        self.parent_scope.as_mut().map_or_else(
                            || error(line, "variable not found"),
                            |parent| {
                                parent.set_var(
                                    name,
                                    &mut new_val.to_vec_literaltype(line),
                                    recurse,
                                    line,
                                );
                            },
                        );
                    }
                } else {
                    let mut new_var: Box<NewList> = match self.get_var(new_name, line) {
                        NewIdentifierType::List(list) => list,
                        _ => error(line, "expected list"),
                    };
                    if name.ends_with(".cdr") {
                        new_var.cdr = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                        };
                    } else {
                        new_var.car = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                        };
                    }
                    self.vars
                        .insert(new_name.to_string(), NewIdentifierType::List(new_var));
                }
            }
            _ => {
                if recurse {
                    if self.has_var(name, false) {
                        self.vars.insert(name.to_string(), new_val);
                    } else {
                        self.parent_scope.as_mut().map_or_else(
                            || error(line, "variable not found"),
                            |parent| parent.set_var(name, value, recurse, line),
                        );
                    }
                } else {
                    self.vars.insert(name.to_string(), new_val);
                }
            }
        }
    }
    pub fn get_var(&mut self, name: &str, line: i32) -> NewIdentifierType {
        // the reason for this being its own method vs using the get method is because it will be easier to use/implemnet getting variable from different scopes
        if name.ends_with(".car") || name.ends_with(".cdr") {
            if let NewIdentifierType::List(list) = self.get_var(&name[..name.len() - 4], line) {
                if name.ends_with(".car") {
                    match list.car {
                        LitOrList::Identifier(list2) => {
                            return NewIdentifierType::List(list2);
                        }
                        LitOrList::Literal(var) => match var {
                            LiteralOrFile::Literal(_) => {
                                return NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                    var,
                                )));
                            }
                            _ => error(line, "expected literal"),
                        },
                    }
                }
                match list.cdr {
                    LitOrList::Identifier(list2) => {
                        return NewIdentifierType::List(list2);
                    }
                    LitOrList::Literal(var) => match var {
                        LiteralOrFile::Literal(_) => {
                            return NewIdentifierType::Vairable(Box::new(NewVairable::new(var)));
                        }
                        _ => error(line, "expected literal"),
                    },
                }
            }
            error(line, "expected list, got something else");
        }
        match self.vars.get(name) {
            Some(v) => match v {
                NewIdentifierType::Vairable(_) => v.clone(),
                NewIdentifierType::List(list) => NewIdentifierType::List(list.clone()),
            },
            None => self.parent_scope.as_mut().map_or_else(
                || error(line, format!("variable not found {name}")),
                |parent| parent.get_var(name, line),
            ),
        }
    }
    pub fn set_function(&mut self, name: char, args: Vec<Thing>, body: f64) {
        self.function.insert(name, (args, body));
    }
    pub fn get_function(&self, name: char) -> Option<(Vec<Thing>, f64)> {
        match self.function.get(&name) {
            Some((args, body)) => Some((args.clone(), *body)),
            None => self
                .parent_scope
                .as_ref()
                .and_then(|parent| parent.get_function(name)),
        }
    }
    pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
        self.vars.remove(name)
    }
    pub fn has_var(&self, name: &str, recurse: bool) -> bool {
        let name: &str = if name.ends_with(".car") || name.ends_with(".cdr") {
            &name[..name.len() - 4]
        } else {
            name
        };
        if !recurse {
            return self.vars.contains_key(name);
        }
        if self.vars.contains_key(name) {
            true
        } else {
            self.parent_scope
                .as_ref()
                .map_or(false, |parent| parent.has_var(name, recurse))
        }
    }
    pub fn drop_scope(&mut self) {
        let p_scope: Self = self
            .parent_scope
            .take()
            .map_or_else(|| error(0, "no parent scope"), |scope| *scope);
        *self = p_scope;
    }

    pub fn from_parent(&mut self) {
        let mut temp_scope = Self::new();
        swap(&mut temp_scope, self);
        *self = Self::new_with_parent(Box::new(temp_scope));
    }
    pub fn has_function(&self, name: char) -> bool {
        self.function.contains_key(&name)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Stopper {
    Break,
    Continue,
    Return(LiteralOrFile),
}

pub struct Eval {
    pub scope: Scope,
    pub in_function: bool,
    pub in_loop: bool,
    pub files: HashMap<String, Rc<RefCell<File>>>,
}

impl Eval {
    pub fn new(mut body: Vec<Thing>) -> Self {
        let mut self_ = Self {
            scope: Scope::new(),
            in_function: false,
            in_loop: false,
            files: HashMap::new(),
        };
        body = self_.find_functions(body);
        self_.find_variables(body);
        self_
    }

    pub fn get_file(&self, name: &str) -> Option<RefMut<'_, File>> {
        self.files.get(name).map(|file| file.borrow_mut())
    }

    pub fn find_functions(&mut self, body: Vec<Thing>) -> Vec<Thing> {
        body.into_iter()
            .filter(|thing| -> bool {
                if let Thing::Function(function) = thing {
                    self.scope.set_function(
                        function.name,
                        function.body.clone(),
                        function.num_arguments,
                    );
                    false
                } else {
                    true
                }
            })
            .collect()
    }
    #[allow(clippy::too_many_lines)]
    pub fn find_variables(&mut self, body: Vec<Thing>) -> Option<Stopper> {
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        for thing in body {
            match thing {
                Thing::Identifier(ref variable) => match variable.value {
                    IdentifierType::Vairable(ref name) => {
                        if let Some(pointer) = self.find_pointer_in_other_stuff(&name.value) {
                            self.scope.set_var(
                                &variable.name,
                                &mut vec![pointer],
                                false,
                                variable.line,
                            );
                        } else {
                            self.scope.set_var(
                                &variable.name,
                                &mut vec![LiteralOrFile::Literal(LiteralType::from_other_stuff(
                                    &name.value,
                                    variable.line,
                                ))],
                                false,
                                variable.line,
                            );
                        }
                    }
                    IdentifierType::List(ref list) => {
                        let car: LiteralOrFile =
                            self.find_pointer_in_other_stuff(&list.car).map_or_else(
                                || {
                                    LiteralOrFile::Literal(LiteralType::from_other_stuff(
                                        &list.car,
                                        variable.line,
                                    ))
                                },
                                |pointer| pointer,
                            );
                        let cdr: LiteralOrFile =
                            self.find_pointer_in_other_stuff(&list.cdr).map_or_else(
                                || {
                                    LiteralOrFile::Literal(LiteralType::from_other_stuff(
                                        &list.cdr,
                                        variable.line,
                                    ))
                                },
                                |pointer| pointer,
                            );
                        self.scope.set_var(
                            &variable.name,
                            &mut vec![car, cdr],
                            false,
                            variable.line,
                        );
                    }
                },
                Thing::Return(os, line) => {
                    let ret: LiteralOrFile =
                        os.map_or(LiteralOrFile::Literal(LiteralType::Hempty), |os| {
                            self.find_pointer_in_other_stuff(&os).map_or_else(
                                || LiteralOrFile::Literal(LiteralType::from_other_stuff(&os, line)),
                                |identifier| identifier,
                            )
                        });
                    return Some(Stopper::Return(ret));
                }
                Thing::Expression(expr) => {
                    let exprs = self.find_pointer_in_stuff(&expr.inside);
                    print!(
                        "{}",
                        NewExpression {
                            inside: exprs,
                            print: expr.print,
                            line: expr.line,
                            new_line: expr.new_line,
                        }
                    );
                }
                Thing::IfStatement(mut if_statement) => {
                    let conditon: LiteralType =
                        match self.find_pointer_in_other_stuff(&if_statement.condition) {
                            Some(pointer) => {
                                info!("if {:?}", pointer);
                                match pointer {
                                    LiteralOrFile::Literal(literal) => literal,
                                    _ => error(if_statement.line, "cannot compare files"),
                                }
                            }
                            None => LiteralType::from_other_stuff(
                                &if_statement.condition,
                                if_statement.line,
                            ),
                        };
                    if conditon.type_eq(&LiteralType::Boolean(true)) {
                    } else {
                        error(if_statement.line, "expected boolean, got something else");
                    }
                    self.scope.from_parent();
                    if conditon == LiteralType::Boolean(true) {
                        if_statement.body_true = self.find_functions(if_statement.body_true);
                        let body_true: Option<Stopper> =
                            self.find_variables(if_statement.body_true);
                        self.scope.drop_scope();
                        if let Some(stop) = body_true {
                            match stop {
                                Stopper::Break | Stopper::Continue => {
                                    if self.in_loop {
                                        return Some(stop);
                                    }
                                    error(if_statement.line, "break or continue outside of loop");
                                }
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error(if_statement.line, "return outside of function");
                                }
                            }
                        }
                    } else {
                        if_statement.body_false = self.find_functions(if_statement.body_false);
                        let z = self.find_variables(if_statement.body_false);
                        self.scope.drop_scope();
                        if let Some(stop) = z {
                            if let Stopper::Return(ret) = stop {
                                if self.in_function {
                                    return Some(Stopper::Return(ret));
                                }
                                error(if_statement.line, "return outside of function");
                            } else {
                                if self.in_loop {
                                    return Some(stop);
                                }
                                error(if_statement.line, "break or continue outside of loop");
                            }
                        }
                    }
                }
                Thing::LoopStatement(loop_statement) => {
                    'l: loop {
                        self.scope.from_parent();
                        let loop_body = self.find_functions(loop_statement.body.clone());
                        self.in_loop = true;
                        let z: Option<Stopper> = self.find_variables(loop_body.clone());
                        self.scope.drop_scope();
                        if let Some(stop) = z {
                            match stop {
                                Stopper::Break => break 'l,
                                Stopper::Continue => continue 'l,
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error(loop_statement.line, "return outside of function");
                                }
                            }
                        }
                    }
                    self.in_loop = false;
                }
                Thing::Break(_) => {
                    return Some(Stopper::Break);
                }
                Thing::Continue(_) => {
                    return Some(Stopper::Continue);
                }
                _ => {}
            }
        }
        None
    }

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<LiteralOrFile> {
        match other_stuff {
            OtherStuff::Identifier(ident) => match self.scope.get_var(&ident.name, ident.line) {
                NewIdentifierType::List(..) => {
                    error(ident.line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => match var.value {
                    LiteralOrFile::Literal(_) => Some(var.value),
                    _ => error(ident.line, "variable is not a literal"),
                },
            },
            OtherStuff::Expression(expr) => Some(self.find_pointer_in_stuff(&expr.inside)),
            _ => None,
        }
    }
    #[allow(clippy::too_many_lines)]
    fn find_pointer_in_stuff(&mut self, stuff: &Stuff) -> LiteralOrFile {
        // need to make ways to extract values from literaltypes/literal/vars easy with function
        match stuff {
            Stuff::Identifier(ident) => match self.scope.get_var(&ident.name, ident.line) {
                NewIdentifierType::List(..) => {
                    error(ident.line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => var.value,
            },
            Stuff::Call(call) => match &call.keyword {
                TokenType::FunctionIdentifier { name } => {
                    if let Some(mut function) = self.scope.get_function(*name) {
                        let mut new_stuff: Vec<LiteralOrFile> = Vec::new();
                        for (index, thing) in call.arguments.iter().enumerate() {
                            arg_error(
                                function.1 as u32,
                                index as u32,
                                &call.keyword,
                                true,
                                call.line,
                            );
                            new_stuff.push(self.find_pointer_in_stuff(thing));
                        }
                        arg_error(
                            function.1 as u32,
                            new_stuff.len() as u32,
                            &call.keyword,
                            false,
                            call.line,
                        );
                        self.scope.from_parent();
                        function.0 = self.find_functions(function.0);
                        self.in_function = true;
                        new_stuff.into_iter().enumerate().for_each(|(i, l)| {
                            self.scope.set_var(
                                format!("${}", i + 1).as_str(),
                                &mut vec![(l)],
                                false,
                                call.line,
                            );
                        });
                        let z: Option<Stopper> = self.find_variables(function.0);
                        self.in_function = false;
                        self.scope.drop_scope();
                        z.map_or(LiteralOrFile::Literal(LiteralType::Hempty), |v| {
                            if let Stopper::Return(a) = v {
                                a
                            } else {
                                error(call.line, "cannot call break/continue at end of function");
                            }
                        })
                    } else {
                        error(call.line, format!("Function {name} is not defined"));
                    }
                }
                TokenType::Type => {
                    arg_error(
                        1,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    match self.find_pointer_in_stuff(&call.arguments[0]) {
                        LiteralOrFile::Literal(a) => {
                            LiteralOrFile::Literal(LiteralType::String(a.get_type()))
                        }
                        LiteralOrFile::File(_) => {
                            LiteralOrFile::Literal(LiteralType::String("file".to_string()))
                        }
                    }
                }
                TokenType::Delete => {
                    if call.arguments.len() != 1 {
                        error(call.line, "delete takes one argument");
                    }
                    if let Stuff::Identifier(ident) = &call.arguments[0] {
                        if self.scope.delete_var(&ident.name).is_some() {
                            LiteralOrFile::Literal(LiteralType::Hempty)
                        } else {
                            error(
                                ident.line,
                                format!("Variable {} is not defined", ident.name),
                            );
                        }
                    } else {
                        error(call.line, "delete only takes a variable name")
                    }
                }
                TokenType::AddWith
                | TokenType::SubtractWith
                | TokenType::DivideWith
                | TokenType::MultiplyWith
                | TokenType::Set => {
                    if let Stuff::Identifier(ident) = &call.arguments[0] {
                        if self.scope.has_var(&ident.name, true) {
                            let mut new_stuff: Vec<LiteralOrFile> = Vec::new();
                            call.arguments.iter().skip(1).for_each(|thing| {
                                new_stuff.push(self.find_pointer_in_stuff(thing));
                            });
                            match new_stuff.len() {
                                1 => {
                                    let literal: LiteralOrFile = new_stuff.remove(0);
                                    let var: LiteralOrFile =
                                        match self.scope.get_var(&ident.name, call.line) {
                                            NewIdentifierType::Vairable(v) => v.value,
                                            NewIdentifierType::List(..) => {
                                                error(ident.line, "Cannot change entire list");
                                            }
                                        };
                                    match call.keyword {
                                        TokenType::Set => {
                                            self.scope.set_var(
                                                &ident.name,
                                                &mut vec![literal.clone()],
                                                true,
                                                call.line,
                                            );
                                            literal
                                        }
                                        TokenType::AddWith => match var {
                                            LiteralOrFile::Literal(LiteralType::Number(num)) => {
                                                if let LiteralOrFile::Literal(
                                                    LiteralType::Number(num2),
                                                ) = literal
                                                {
                                                    let new_val = num + num2;
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &mut vec![LiteralOrFile::Literal(
                                                            LiteralType::Number(new_val),
                                                        )],
                                                        true,
                                                        call.line,
                                                    );
                                                    LiteralOrFile::Literal(LiteralType::Number(
                                                        new_val,
                                                    ))
                                                } else {
                                                    error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            }
                                            LiteralOrFile::Literal(LiteralType::String(mut s)) => {
                                                match literal {
                                                    LiteralOrFile::Literal(
                                                        LiteralType::String(s2),
                                                    ) => {
                                                        s.push_str(s2.as_str());
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::String(s.clone()),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(
                                                        LiteralType::Number(n),
                                                    ) => {
                                                        s.push_str(&n.to_string());
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::String(s.clone()),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(
                                                        LiteralType::Boolean(boolean),
                                                    ) => {
                                                        s.push_str(&boolean.to_string());
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::String(s.clone()),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(LiteralType::Hempty) => {
                                                        s.push_str("hempty");
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::String(s.clone()),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::String(
                                                            s,
                                                        ))
                                                    }
                                                    _ => {
                                                        error(
                                                            call.line,
                                                            format!(
                                                                "Variable {} is not a string",
                                                                ident.name
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                            _ => {
                                                error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a number/string",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        },
                                        TokenType::MultiplyWith => match var {
                                            LiteralOrFile::Literal(LiteralType::Number(num)) => {
                                                if let LiteralOrFile::Literal(
                                                    LiteralType::Number(num2),
                                                ) = literal
                                                {
                                                    let new_val: f64 = num * num2;
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &mut vec![LiteralOrFile::Literal(
                                                            LiteralType::Number(new_val),
                                                        )],
                                                        true,
                                                        call.line,
                                                    );
                                                    LiteralOrFile::Literal(LiteralType::Number(
                                                        new_val,
                                                    ))
                                                } else {
                                                    error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            }
                                            LiteralOrFile::Literal(LiteralType::String(ref s)) => {
                                                if let LiteralOrFile::Literal(
                                                    LiteralType::Number(num),
                                                ) = literal
                                                {
                                                    let new_string: String = (0..num as i32)
                                                        .map(|_| s.to_string())
                                                        .collect();
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &mut vec![LiteralOrFile::Literal(
                                                            LiteralType::String(new_string.clone()),
                                                        )],
                                                        true,
                                                        call.line,
                                                    );
                                                    LiteralOrFile::Literal(LiteralType::String(
                                                        new_string,
                                                    ))
                                                } else {
                                                    error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            }
                                            _ => {
                                                error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a number/string",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        },
                                        TokenType::SubtractWith | TokenType::DivideWith => {
                                            if let LiteralOrFile::Literal(LiteralType::Number(
                                                nums,
                                            )) = var
                                            {
                                                if let LiteralOrFile::Literal(
                                                    LiteralType::Number(num),
                                                ) = literal
                                                {
                                                    if call.keyword == TokenType::SubtractWith {
                                                        let new_val = nums - num;
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::Number(new_val),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::Number(
                                                            new_val,
                                                        ))
                                                    } else {
                                                        let new_val = nums / num;
                                                        self.scope.set_var(
                                                            &ident.name,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralType::Number(new_val),
                                                            )],
                                                            true,
                                                            call.line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralType::Number(
                                                            new_val,
                                                        ))
                                                    }
                                                } else {
                                                    error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            } else {
                                                error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a number/string",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        }
                                        _ => {
                                            error(
                                                call.line,
                                                format!(
                                                    "Invalid operator for literal {}",
                                                    call.keyword
                                                ),
                                            );
                                        }
                                    }
                                }
                                2 => {
                                    if call.keyword == TokenType::Set {
                                        self.scope.set_var(
                                            &ident.name,
                                            &mut new_stuff,
                                            true,
                                            call.line,
                                        );
                                        LiteralOrFile::Literal(LiteralType::String(
                                            new_stuff
                                                .iter()
                                                .map(std::string::ToString::to_string)
                                                .collect(),
                                        ))
                                    } else {
                                        error(
                                            call.line,
                                            format!(
                                                "Too many arguments for function {}",
                                                call.keyword
                                            )
                                            .as_str(),
                                        );
                                    }
                                }
                                _ => error(
                                    call.line,
                                    format!("Too many arguments for function {}", call.keyword)
                                        .as_str(),
                                ),
                            }
                        } else {
                            error(
                                ident.line,
                                format!("Variable {} is not defined", ident.name),
                            );
                        }
                    } else {
                        error(
                            call.line,
                            format!("First argument of {} must be an identifier", call.keyword)
                                .as_str(),
                        );
                    }
                }
                TokenType::Open => {
                    arg_error(
                        1,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // check if the first argument is a string
                    let arg = self
                        .find_pointer_in_stuff(&call.arguments[0])
                        .get_string(call.line, &call.keyword);
                    if std::path::Path::new(&arg).exists() {
                        LiteralOrFile::File(arg)
                    } else {
                        error(
                            call.line,
                            format!("Could not open file {arg}: does not exist").as_str(),
                        );
                    }
                }
                TokenType::Close | TokenType::Read => {
                    arg_error(
                        1,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // evalute args[0] and check if it is a file
                    match &call.arguments[0] {
                        Stuff::Identifier(ident) => {
                            let files = self.scope.get_var(&ident.name, call.line);
                            match files {
                                NewIdentifierType::Vairable(var) => {
                                    match var.value {
                                        LiteralOrFile::File(file) => {
                                            // set idnetifier to nothing
                                            match call.keyword {
                                                TokenType::Close => {
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &mut vec![LiteralOrFile::Literal(
                                                            LiteralType::Hempty,
                                                        )],
                                                        true,
                                                        call.line,
                                                    );
                                                }
                                                TokenType::Read => {
                                                    let contents = // create a string to hold the contents of the file
                                                    match read_file(&file) {
                                                        Ok(contents) => contents,
                                                        Err(err) => {
                                                            error(call.line, format!("{err}"));
                                                        }
                                                    }; // read the file into the string
                                                    return LiteralOrFile::Literal(
                                                        LiteralType::String(contents),
                                                    ); // return the string
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => error(
                                            call.line,
                                            format!("{} is not a file", ident.name).as_str(),
                                        ),
                                    }
                                }
                                NewIdentifierType::List(_) => error(
                                    call.line,
                                    format!("Variable {} is not a file", ident.name).as_str(),
                                ),
                            }
                        }
                        other => {
                            match self.find_pointer_in_stuff(other) {
                                LiteralOrFile::File(file) => {
                                    match call.keyword {
                                        TokenType::Close => {
                                            self.scope.files.remove(&file);
                                        }
                                        TokenType::Read => {
                                            let contents: String = String::new(); // create a string to hold the contents of the file
                                            match read_file(&file) {
                                                Ok(contents) => contents,
                                                Err(err) => {
                                                    error(call.line, format!("{err}"));
                                                }
                                            }; // read the file into the string
                                            return LiteralOrFile::Literal(LiteralType::String(
                                                contents,
                                            )); // return the string
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {
                                    error(
                                        call.line,
                                        format!(
                                            "First argument of {} must be a file",
                                            call.keyword
                                        )
                                        .as_str(),
                                    );
                                }
                            };
                        }
                    }
                    LiteralOrFile::Literal(LiteralType::Hempty)
                }
                TokenType::Write => {
                    //takes 3 arguments:
                    // 1. the file
                    // 2. the string to write
                    // 3. the mode (a: append or w: overwrite)
                    arg_error(
                        3,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // get the file
                    let file = self
                        .find_pointer_in_stuff(&call.arguments[0])
                        .get_file(call.line, &call.keyword);
                    // get the string
                    let string = self
                        .find_pointer_in_stuff(&call.arguments[1])
                        .get_string(call.line, &call.keyword);
                    // get the mode
                    let mode = self
                        .find_pointer_in_stuff(&call.arguments[2])
                        .get_string(call.line, &call.keyword);
                    // write the string to the file
                    match write_file(&file, &string, &mode) {
                        Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
                        Err(err) => {
                            error(call.line, format!("{err}").as_str());
                        }
                    }
                }
                TokenType::ReadLine => {
                    // takes 2 arguments: file, line
                    arg_error(
                        2,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // get the file
                    let file = self
                        .find_pointer_in_stuff(&call.arguments[0])
                        .get_file(call.line, &call.keyword);
                    // get the line
                    let line = self
                        .find_pointer_in_stuff(&call.arguments[1])
                        .get_number(call.line, &call.keyword);
                    // read the the file
                    match read_file(&file) {
                        Ok(contents) => {
                            let lines = contents.split('\n').collect::<Vec<&str>>();
                            if line as usize > lines.len() {
                                error(
                                    call.line,
                                    format!("Line {line} does not exist in file {file}"),
                                );
                            }
                            LiteralOrFile::Literal(LiteralType::String(
                                lines[line as usize - 1].to_string(),
                            ))
                        }
                        Err(err) => {
                            error(call.line, format!("{err}").as_str());
                        }
                    }
                }
                TokenType::WriteLine => {
                    // takes 4 arguments: file, line, string and mode (a: append to beging of line or w: overwrite)
                    arg_error(
                        4,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // get the file
                    let file = self
                        .find_pointer_in_stuff(&call.arguments[0])
                        .get_file(call.line, &call.keyword);
                    // get the string
                    let mut string = self
                        .find_pointer_in_stuff(&call.arguments[1])
                        .get_string(call.line, &call.keyword);
                    // get the line
                    let line = self
                        .find_pointer_in_stuff(&call.arguments[2])
                        .get_number(call.line, &call.keyword);
                    // get the mode
                    let mode = self
                        .find_pointer_in_stuff(&call.arguments[3])
                        .get_string(call.line, &call.keyword);
                    // read the file
                    let mut contents = match read_file(&file) {
                        Ok(contents) => contents,
                        Err(err) => {
                            error(call.line, format!("{err}").as_str());
                        }
                    };
                    // split the contents into lines
                    let mut lines = contents.split('\n').collect::<Vec<&str>>();
                    // if the line is greater than the number of lines, add a new line
                    if line as usize > lines.len() {
                        error(call.line, "Line does not exist in file");
                    }
                    string = match mode.as_str() {
                        "a" => {
                            format!("{string}{}", lines[line as usize - 1],)
                        }
                        "w" => string,
                        _ => {
                            error(
                                call.line,
                                format!("Mode {mode} is not a valid mode").as_str(),
                            );
                        }
                    };
                    lines[line as usize - 1] = string.as_str();
                    // collect all lines
                    contents = lines.join("\n");
                    // write the file
                    match write_file(&file, &contents, "w") {
                        Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
                        Err(err) => {
                            error(call.line, format!("{err}").as_str());
                        }
                    }
                }
                TokenType::DeleteFile | TokenType::CreateFile => {
                    // takes 1 argument: file
                    arg_error(
                        1,
                        call.arguments.len() as u32,
                        &call.keyword,
                        false,
                        call.line,
                    );
                    // get the file
                    let file = self
                        .find_pointer_in_stuff(&call.arguments[0])
                        .get_file(call.line, &call.keyword);
                    // match delete or create file
                    match call.keyword {
                        TokenType::DeleteFile => match fs::remove_file(&file) {
                            Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
                            Err(err) => {
                                error(call.line, format!("{err}").as_str());
                            }
                        },
                        TokenType::CreateFile => {
                            // create the file
                            match OpenOptions::new().create(true).open(&file) {
                                Ok(_) => LiteralOrFile::File(file),
                                Err(err) => {
                                    error(call.line, format!("{err}").as_str());
                                }
                            }
                        }
                        _ => LiteralOrFile::Literal(LiteralType::Hempty),
                    }
                }
                t => {
                    let mut new_stuff: Vec<LiteralType> = Vec::new();
                    call.arguments.iter().for_each(|thing| {
                        match self.find_pointer_in_stuff(thing) {
                            LiteralOrFile::Literal(literal) => {
                                new_stuff.push(literal);
                            }
                            LiteralOrFile::File(_) => {
                                error(
                                    call.line,
                                    format!(
                                        "Cannot use file as argument for function {}",
                                        call.keyword
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    });
                    LiteralOrFile::Literal(t.r#do(&new_stuff, call.line))
                }
            },
            Stuff::Literal(lit) => LiteralOrFile::Literal(lit.literal.clone()),
        }
    }
}

impl fmt::Debug for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope)?;
        Ok(())
    }
}
