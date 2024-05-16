#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(static_mut_refs)]
#![deny(clippy::use_self, rust_2018_idioms, clippy::missing_panics_doc)]
use core::panic;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::iter::Peekable;
use std::str::Chars;
use std::{cell::RefCell, cmp::Ordering};
use std::{collections::BTreeSet, rc::Rc};
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Scope(usize);
#[derive(Debug)]
struct UniqueNumberManager(usize);

// static mut UNIQUE_NUMBER_GENERATOR: usize = 1;
// fn unique_number_generator() -> usize {
//     unsafe {
//         let current = UNIQUE_NUMBER_GENERATOR;
//         UNIQUE_NUMBER_GENERATOR += 1;
//         current
//     }
// }
impl UniqueNumberManager {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> usize {
        let current = self.0;
        self.0 += 1;
        current
    }

    fn new_scope(&mut self) -> Scope {
        Scope(self.next())
    }

    fn gen_sym(&mut self, name: impl ToString) -> Symbol {
        Symbol(name.to_string().into(), self.next())
    }
}

type ScopeSet = BTreeSet<Scope>;
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Syntax(Symbol, ScopeSet);

trait AdjustScope: Sized {
    fn adjust_scope(
        self,
        other_scope: Scope,
        operation: fn(ScopeSet, Scope) -> BTreeSet<Scope>,
    ) -> Self;

    fn add_scope(self, other_scope: Scope) -> Self {
        self.adjust_scope(other_scope, |mut scopes, other_scope| {
            scopes.insert(other_scope);
            scopes
        })
    }

    fn flip_scope(self, other_scope: Scope) -> Self {
        self.adjust_scope(other_scope, |mut scopes, other_scope| {
            if !scopes.remove(&other_scope) {
                scopes.insert(other_scope);
            }
            scopes
        })
    }
}

impl Syntax {
    #[must_use]
    pub fn new(symbol: Symbol) -> Self {
        Self(symbol, BTreeSet::new())
    }
}

impl AdjustScope for Syntax {
    fn adjust_scope(
        self,
        other_scope_set: Scope,
        operation: fn(ScopeSet, Scope) -> BTreeSet<Scope>,
    ) -> Self {
        Self(self.0, operation(self.1, other_scope_set))
    }
}

pub type AnalyzedResult = Result<Box<dyn AnalyzeFn>, String>;
pub trait AnalyzeFn: Fn(EnvRef) -> Result<Ast, String> {
    fn clone_box<'a>(&self) -> Box<dyn 'a + AnalyzeFn>
    where
        Self: 'a;
}

impl<F> AnalyzeFn for F
where
    F: Fn(EnvRef) -> Result<Ast, String> + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + AnalyzeFn>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}
impl<'a> Clone for Box<dyn 'a + AnalyzeFn> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Function {
    Lambda(Lambda),
    Primitive(Primitive),
}

impl Function {
    fn apply(&self, args: Vec<Ast>) -> Result<Ast, String> {
        match self {
            Self::Lambda(Lambda(body, env, params)) => {
                let env = Env::extend_envoirnment(env.clone(), params, args)?;
                body(env)
            }
            Self::Primitive(p) => p(args),
        }
    }
}

#[derive(Clone)]
pub struct Lambda(Box<dyn AnalyzeFn>, EnvRef, Vec<Symbol>);

impl PartialEq for Lambda {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(procedure)")
    }
}
type Primitive = fn(Vec<Ast>) -> Result<Ast, String>;
#[derive(Clone, PartialEq, Debug)]
pub enum Ast {
    List(Vec<Ast>),
    Syntax(Syntax),
    Number(f64),
    Symbol(Symbol),
    Function(Function),
}

impl Ast {
    pub fn datum_to_syntax(self) -> Self {
        match self {
            Self::List(l) => Self::List(l.into_iter().map(Self::datum_to_syntax).collect()),
            Self::Syntax(_) => self,
            Self::Symbol(s) => Self::Syntax(Syntax::new(s)),
            _ => self,
        }
    }
    fn syntax_to_datum(self) -> Self {
        match self {
            Self::List(l) => Self::List(l.into_iter().map(Self::syntax_to_datum).collect()),
            Self::Syntax(Syntax(s, _)) => Self::Symbol(s),
            _ => self,
        }
    }
    fn identifier(&self) -> bool {
        matches!(self, Self::Syntax(_))
    }
}

impl AdjustScope for Ast {
    fn adjust_scope(
        self,
        other_scope: Scope,
        operation: fn(ScopeSet, Scope) -> BTreeSet<Scope>,
    ) -> Self {
        match self {
            Self::List(l) => Self::List(
                l.into_iter()
                    .map(|e| e.adjust_scope(other_scope, operation))
                    .collect(),
            ),
            Self::Syntax(s) => Self::Syntax(s.adjust_scope(other_scope, operation)),
            _ => self,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Binding {
    Lambda,
    LetSyntax,
    Quote,
    QuoteSyntax,
    // DatumToSyntax,
    // SyntaxToDatum,
    // SyntaxE,
    // List,
    // Cons,
    // Car,
    // Cdr,
    // Map,
    Variable(Symbol),
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variable(Symbol(s, i)) => format!("{s}{i}"),
                Self::Lambda => "lambda".to_string(),
                Self::LetSyntax => "let-syntax".to_string(),
                Self::Quote => "quote".to_string(),
                Self::QuoteSyntax => "quote-syntax".to_string(),
                // Self::DatumToSyntax => "datum->syntax".to_owned(),
                // Self::SyntaxToDatum => "syntax->datum".to_owned(),
                // Self::SyntaxE => "syntax-e".to_string(),
                // Self::List => "list".to_string(),
                // Self::Cons => "cons".to_string(),
                // Self::Car => "car".to_string(),
                // Self::Cdr => "cdr".to_string(),
                // Self::Map => "map".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Expander<T> {
    all_bindings: HashMap<Syntax, T>,
    core_forms: BTreeSet<Binding>,
    core_primitives: BTreeSet<Binding>,
    core_scope: Scope,
    scope_creator: UniqueNumberManager,
}

impl Default for Expander<Binding> {
    fn default() -> Self {
        Self::new()
    }
}

impl Expander<Binding> {
    #[must_use]
    pub fn new() -> Self {
        let mut scope_creator = UniqueNumberManager::new();
        let core_scope = scope_creator.new_scope();
        let core_forms = BTreeSet::from([
            Binding::Lambda,
            Binding::LetSyntax,
            Binding::Quote,
            Binding::QuoteSyntax,
        ]);
        let core_primitives = BTreeSet::from([
            Binding::Variable("datum->syntax".into()),
            Binding::Variable("syntax->datum".into()),
            Binding::Variable("list".into()),
            Binding::Variable("cons".into()),
            Binding::Variable("car".into()),
            Binding::Variable("cdr".into()),
            Binding::Variable("map".into()),
        ]);
        let mut this = Self {
            scope_creator,
            core_scope,
            core_primitives,
            core_forms,
            all_bindings: HashMap::new(),
        };
        this.core_forms
            .clone()
            .union(&this.core_primitives.clone())
            .for_each(|core| {
                this.add_binding(
                    Syntax(
                        Symbol(core.to_string().into(), 0),
                        BTreeSet::from([this.core_scope]),
                    ),
                    core.clone(),
                );
            });
        this
    }
    fn add_binding(&mut self, id: Syntax, binding: Binding) {
        self.all_bindings.insert(id, binding);
    }

    fn add_local_binding(&mut self, id: Syntax) -> Symbol {
        let symbol = self.scope_creator.gen_sym(&id.0);
        self.add_binding(id, Binding::Variable(symbol.clone()));
        symbol
    }

    fn resolve(&self, id: &Syntax) -> Option<&Binding> {
        let candidate_ids = self.find_all_matching_bindings(id);
        candidate_ids
            .clone()
            .max_by_key(|id| id.1.len())
            .filter(|id| check_unambiguous(id, candidate_ids))
            .and_then(|id| self.all_bindings.get(id))
    }

    fn find_all_matching_bindings<'a>(
        &'a self,
        id: &'a Syntax,
    ) -> impl Iterator<Item = &Syntax> + Clone + 'a {
        self.all_bindings
            .keys()
            .filter(move |c_id| c_id.0 == id.0 && c_id.1.is_subset(&id.1))
    }

    pub fn introduce<T: AdjustScope>(&self, s: T) -> T {
        s.add_scope(self.core_scope)
    }

    pub fn expand(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        match s {
            Ast::List(l) if matches!(&l[..], [Ast::Syntax(_), ..]) => {
                self.expand_id_application_form(l, env)
            }
            Ast::Syntax(s) => self.expand_identifier(s, env),
            Ast::Number(_) => todo!(),
            Ast::Symbol(_) => todo!(),
            Ast::List(_) => todo!(),
            Ast::Function(_) => todo!(),
        }
    }

    fn expand_identifier(&self, s: Syntax, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let binding = self.resolve(&s).ok_or(format!("free variable {}", s.0))?;
        if self.core_forms.contains(binding) {
            Err(format!("bad syntax {}", s.0))
        } else if self.core_primitives.contains(binding) {
            Ok(Ast::Syntax(s))
        } else {
            let Binding::Variable(binding) = binding else {
                panic!()
            };
            let v = env
                .lookup(binding)
                .ok_or(format!("out of context {}", s.0))?;
            if let CompileTimeBinding::Symbol(_) = v {
                Ok(Ast::Syntax(s))
            } else {
                Err(format!("bad syntax {}", s.0))
            }
        }
    }

    // constraints = s.len() > 0
    // constraints = s[0] == Ast::Syntax(_)
    fn expand_id_application_form(
        &mut self,
        s: Vec<Ast>,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        let Some(Ast::Syntax(id)) = s.first() else {
            unreachable!()
        };
        let binding = self.resolve(id).ok_or(format!("free variable {}", id.0))?;
        match binding {
            Binding::Lambda => self.expand_lambda(s, env),
            Binding::LetSyntax => self.expand_let_syntax(s, env),
            Binding::Quote | Binding::QuoteSyntax => match &s[..] {
                [_, _] => Ok(Ast::List(s)),
                _ => Err(format!("bad syntax {s:?}")),
            },
            Binding::Variable(binding) => {
                let v = env
                    .lookup(binding)
                    .ok_or(format!("out of context {}", binding.0))?;
                match v {
                    CompileTimeBinding::Symbol(_) => self.expand_app(s, env),
                    CompileTimeBinding::Macro(m) => {
                        let apply_transformer = self.apply_transformer(m, Ast::List(s));
                        self.expand(apply_transformer?, env)
                    }
                }
            }
        }
    }

    fn expand_app(&mut self, s: Vec<Ast>, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        s.into_iter()
            .map(|sub_s| self.expand(sub_s, env.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map(Ast::List)
    }

    fn expand_lambda(&mut self, s: Vec<Ast>, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let [lambda_id, Ast::List(arg), body] = &s[..] else {
            Err(format!("invalid syntax {s:?}"))?
        };
        let [Ast::Syntax(arg_id)] = &arg[..] else {
            Err(format!("invalid syntax {s:?}"))?
        };
        let sc = self.scope_creator.new_scope();
        let id = arg_id.clone().add_scope(sc);
        let binding = self.add_local_binding(id.clone());
        let body_env = env.extend(binding.clone(), CompileTimeBinding::Symbol(binding));
        let exp_body = self.expand(body.clone().add_scope(sc), body_env)?;
        Ok(Ast::List(vec![
            lambda_id.clone(),
            Ast::List(vec![Ast::Syntax(id)]),
            exp_body,
        ]))
    }

    fn expand_let_syntax(
        &mut self,
        s: Vec<Ast>,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        let [_let_syntax_id, Ast::List(arg), body] = &s[..] else {
            Err(format!("invalid syntax {s:?}"))?
        };
        let [Ast::Syntax(lhs_id), rhs] = &arg[..] else {
            Err(format!("invalid syntax {s:?}"))?
        };
        let sc = self.scope_creator.new_scope();
        let id = lhs_id.clone().add_scope(sc);
        let binding = self.add_local_binding(id);
        let rhs_val = self.eval_for_syntax_binding(rhs.clone())?;
        let body_env = env.extend(binding, rhs_val);
        self.expand(body.clone().add_scope(sc), body_env)
    }

    fn eval_for_syntax_binding(&mut self, rhs: Ast) -> Result<CompileTimeBinding, String> {
        let var_name = format!("problem `evaluating` macro {rhs:?}");
        let expand = self.expand(rhs, CompileTimeEnvoirnment::new());
        self.eval_compiled(self.compile(expand?).ok_or(var_name)?)
    }

    fn compile(&self, rhs: Ast) -> Option<Ast> {
        match rhs {
            Ast::List(l) => {
                let s = l.first()?;
                let core_sym = if let Ast::Syntax(s) = s {
                    self.resolve(s)
                } else {
                    None
                };
                match core_sym {
                    Some(Binding::Lambda) => {
                        let [_, Ast::List(arg), body] = &l[..] else {
                            None?
                        };
                        let [Ast::Syntax(id)] = &arg[..] else { None? };
                        let Binding::Variable(id) = self.resolve(id)? else {
                            None?
                        };
                        Some(Ast::List(vec![
                            Ast::Symbol(Symbol("lambda".into(), 0)),
                            Ast::List(vec![Ast::Symbol(id.clone())]),
                            self.compile(body.clone())?,
                        ]))
                    }
                    Some(Binding::Quote) => {
                        if let [_, datum] = &l[..] {
                            Some(Ast::List(vec![
                                Ast::Symbol(Symbol("quote".into(), 0)),
                                datum.clone().syntax_to_datum(),
                            ]))
                        } else {
                            None
                        }
                    }
                    Some(Binding::QuoteSyntax) => {
                        if let [_, datum] = &l[..] {
                            Some(Ast::List(vec![
                                Ast::Symbol(Symbol("quote".into(), 0)),
                                datum.clone(),
                            ]))
                        } else {
                            None
                        }
                    }
                    _ => Some(Ast::List(
                        l.into_iter()
                            .map(|e| self.compile(e))
                            .collect::<Option<Vec<_>>>()?,
                    )),
                }
            }
            Ast::Syntax(s) => {
                if let Binding::Variable(s) = self.resolve(&s)? {
                    Some(Ast::Symbol(s.clone()))
                } else {
                    None
                }
            }
            Ast::Number(_) => Some(rhs),
            Ast::Symbol(_) => todo!(),
            Ast::Function(_) => todo!(),
        }
    }

    fn eval_compiled(&self, new: Ast) -> Result<CompileTimeBinding, String> {
        let env = Env::new();
        env.borrow_mut().define(
            Symbol("datum->syntax".into(), 0),
            Ast::Function(Function::Primitive(move |e| {
                let [e] = &e[..] else {
                    Err(format!("arity error: expected 1 argument, got {}", e.len()))?
                };
                Ok(e.clone().datum_to_syntax())
            })),
        );
        env.borrow_mut().define(
            Symbol("syntax->datum".into(), 0),
            Ast::Function(Function::Primitive(move |e| {
                let [e] = &e[..] else {
                    Err(format!("arity error: expected 1 argument, got {}", e.len()))?
                };
                Ok(e.clone().syntax_to_datum())
            })),
        );
        env.borrow_mut().define(
            Symbol("syntax-e".into(), 0),
            Ast::Function(Function::Primitive(move |e| {
                let [Ast::Syntax(Syntax(e, _))] = &e[..] else {
                    Err(format!("arity error: expected 1 argument, got {}", e.len()))?
                };
                Ok(Ast::Symbol(e.clone()))
            })),
        );
        Evaluator::eval(new, env).and_then(|e| {
            if let Ast::Function(f) = e {
                Ok(CompileTimeBinding::Macro(f))
            } else {
                Err(format!("{e:?} is not a macro"))
            }
        })
    }

    fn apply_transformer(&mut self, m: Function, s: Ast) -> Result<Ast, String> {
        let intro_scope = self.scope_creator.new_scope();
        let intro_s = s.add_scope(intro_scope);
        let transformed_s = m.apply(vec![intro_s])?;

        Ok(transformed_s.flip_scope(intro_scope))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Env {
    scope: HashMap<Symbol, Ast>,
    parent: Option<EnvRef>,
}

impl Env {
    fn lookup(&self, symbol: &Symbol) -> Option<Ast> {
        self.scope.get(symbol).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().lookup(symbol))
        })
    }
    fn set(&mut self, symbol: Symbol, mut expr: Ast) -> Option<Ast> {
        {
            match self.scope.get_mut(&symbol) {
                Some(s) => {
                    std::mem::swap(s, &mut expr);
                    Some(expr)
                }
                None => self
                    .parent
                    .as_ref()
                    .and_then(|parent| parent.borrow_mut().set(symbol, expr)),
            }
        }
    }
    fn define(&mut self, symbol: Symbol, expr: Ast) -> bool {
        self.scope.insert(symbol, expr).is_some()
    }
    fn new_scope(env: EnvRef) -> EnvRef {
        let parent = Some(env);
        let scope = HashMap::new();
        Rc::new(RefCell::new(Self { scope, parent }))
    }

    fn extend_envoirnment(
        env: EnvRef,
        params: &[Symbol],
        args: Vec<Ast>,
    ) -> Result<EnvRef, String> {
        let new_envoirnment = Self::new_scope(env);
        match params.len().cmp(&args.len()) {
            Ordering::Less => Err("arity error to many arguments passed in".to_string()),
            Ordering::Greater => Err("arity error to little arguments passed in".to_string()),
            Ordering::Equal => {
                for (param, arg) in params.iter().zip(args.into_iter()) {
                    new_envoirnment.borrow_mut().define(param.clone(), arg);
                }
                Ok(new_envoirnment)
            }
        }
    }

    fn new() -> EnvRef {
        let scope = HashMap::new();
        // TODO: primitive environment
        let parent = None;
        Rc::new(RefCell::new(Self { scope, parent }))
    }
}

type EnvRef = Rc<RefCell<Env>>;

pub struct Evaluator {}

impl Evaluator {
    fn eval(expr: Ast, env: EnvRef) -> Result<Ast, String> {
        Self::analyze(expr)?(env)
    }
    fn analyze(expr: Ast) -> AnalyzedResult {
        match expr {
            Ast::List(list) => match list.first() {
                Some(Ast::Symbol(Symbol(lambda, 0))) if **lambda == *"lambda" => {
                    let mut list = list.clone().into_iter();
                    let (Some(Ast::List(arg)), Some(body), None) =
                        (list.next(), list.next(), list.next())
                    else {
                        Err(format!("bad syntax {list:?}"))?
                    };
                    let args = arg
                        .into_iter()
                        .map(|arg| {
                            if let Ast::Symbol(s) = arg {
                                Ok(s)
                            } else {
                                Err(format!("bad syntax {arg:?} is not a valid paramter"))
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let body = Self::analyze(body)?;
                    Ok(Box::new(move |env| {
                        Ok(Ast::Function(Function::Lambda(Lambda(
                            body.clone(),
                            env,
                            args.clone(),
                        ))))
                    }))
                }
                Some(Ast::Symbol(Symbol(quote, 0))) if **quote == *"quote" => {
                    if list.len() == 2 {
                        Ok(Box::new(move |_| Ok(list[1].clone())))
                    } else {
                        Err(format!("bad syntax {list:?}"))
                    }
                }
                Some(f) => {
                    let f = Self::analyze(f.clone())?;
                    let rest = list[1..]
                        .iter()
                        .cloned()
                        .map(Self::analyze)
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Box::new(move |env| {
                        Self::execute_application(
                            f.clone()(env.clone())?,
                            rest.clone()
                                .into_iter()
                                .map(|a| a(env.clone()))
                                .collect::<Result<Vec<_>, _>>()?,
                        )
                    }))
                }
                None => Err(format!("bad syntax {list:?}")),
            },
            Ast::Symbol(s) => Ok(Box::new(move |env| {
                env.borrow()
                    .lookup(&s)
                    .ok_or(format!("free variable {s:?}"))
            })),
            _ => Ok(Box::new(move |_| Ok(expr.clone()))),
        }
    }

    fn execute_application(f: Ast, args: Vec<Ast>) -> Result<Ast, String> {
        if let Ast::Function(f) = f {
            f.apply(args)
        } else {
            Err(format!(
                "cannot not apply {f:?} to {args:?}, because {f:?} is not a function"
            ))
        }
    }
}

pub struct Reader(String);

type Input<'a> = Peekable<Chars<'a>>;
type ReaderInnerResult<'a> = Result<(Ast, Input<'a>), (String, Input<'a>)>;
impl Reader {
    pub fn read(&mut self) -> Result<Ast, String> {
        let input = self.0.chars().peekable();
        match Self::read_inner(input) {
            Ok((ast, rest)) => {
                self.0 = rest.collect();
                Ok(ast)
            }
            Err((reason, rest)) => {
                self.0 = rest.collect();
                Err(reason)
            }
        }
    }

    fn read_inner(input: Input<'_>) -> ReaderInnerResult<'_> {
        let mut input = Self::read_whitespace_and_comments(input).1;
        match input.peek() {
            Some('(') => Self::read_list(input),
            Some(')') => {
                input.next();
                Err(("unfinished pair".to_string(), input))
            }

            Some(n) if n.is_ascii_digit() => Self::read_number(input),
            Some(_) => Self::read_symbol(input),
            None => Err((String::from("empty input"), input)),
        }
    }

    fn read_whitespace_and_comments(mut input: Input<'_>) -> (bool, Input<'_>) {
        let mut found = false;
        while let Some(c) = input.peek() {
            match c {
                ';' => {
                    found = true;
                    // we do find to skip untill we find newline, this is essentially
                    // what skip while does, but skip while returns a new type and we
                    // cannot do impl trait in type alias so this does not work for with
                    // my input type
                    input.find(|c| *c != '\n');
                }
                c if c.is_whitespace() => {
                    found = true;
                    input.next();
                }
                _ => break,
            }
        }
        (found, input)
    }

    // parse symbol if not followed by space paren or comment
    // invariant Some('.') | Some(c) if c.is_ascci_digit() = input.peek()
    fn read_number(input: Input<'_>) -> ReaderInnerResult<'_> {
        let (first, mut input) = Self::read_digit(input);
        let (second, input) = {
            if input.peek().copied().filter(|c| *c == '.').is_some() {
                input.next();
                let (digits, input) = Self::read_digit(input);
                (format!(".{digits}"), input)
            } else {
                (String::new(), input)
            }
        };
        let (last, input) = Self::read_symbol_inner(input);
        match (first.as_str(), second.as_str(), last.as_str()) {
            ("", "." | "", "") => Err(("invalid syntax dangling dot".to_owned(), input)),
            (_, _, "") => match format!("{first}{second}").parse::<f64>() {
                Ok(n) => Ok((Ast::Number(n), input)),
                Err(e) => Err((e.to_string(), input)),
            },

            (first, second, _) => {
                let (last, input) = Self::read_symbol_inner(input);
                Ok((
                    Ast::Symbol(Symbol(format!("{first}{second}{last}").into(), 0)),
                    input,
                ))
            }
        }
    }
    fn read_digit(mut input: Input<'_>) -> (String, Input<'_>) {
        let mut number = String::new();
        while let Some(n) = input.next().filter(char::is_ascii_digit) {
            input.next();
            number.push(n);
        }
        (number, input)
    }
    // constraints input.next() == Some(c) if c != whitespace or comment or paren
    fn read_symbol(input: Input<'_>) -> ReaderInnerResult<'_> {
        let (symbol, input) = Self::read_symbol_inner(input);
        Ok((Ast::Symbol(Symbol(symbol.into(), 0)), input))
    }

    // invariant input.next() == Some('(')
    fn read_list(mut input: Input<'_>) -> ReaderInnerResult<'_> {
        input.next();
        let mut list = vec![];
        loop {
            input = Self::read_whitespace_and_comments(input).1;
            match input.peek() {
                // TODO: dot tailed list and pair instead of list
                Some(')') => {
                    input.next();
                    break Ok((Ast::List(list), input));
                }
                Some(_) => {
                    let item: Ast;
                    (item, input) = Self::read_inner(input)?;
                    list.push(item);
                }
                None => {
                    input.next();
                    break Err(("unfinished list".to_string(), input));
                }
            }
        }
    }

    fn read_symbol_inner(mut input: Input<'_>) -> (String, Input<'_>) {
        let mut str = String::new();
        while let Some(char) = input.peek() {
            if char.is_whitespace() || ['(', ')', ';', '"', '\''].contains(char) {
                break;
            }
            str.push(*char);
        }
        (str, input)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub Rc<str>, pub usize);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl From<Rc<str>> for Symbol {
    fn from(value: Rc<str>) -> Self {
        Self(value, 0)
    }
}
impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self(value.into(), 0)
    }
}

#[derive(Clone)]
pub enum CompileTimeBinding {
    Symbol(Symbol),
    Macro(Function),
}

#[derive(Clone)]
struct CompileTimeEnvoirnment(HashMap<Symbol, CompileTimeBinding>);

impl CompileTimeEnvoirnment {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extend(&self, key: Symbol, value: CompileTimeBinding) -> Self {
        let mut map = self.0.clone();
        map.insert(key, value);
        Self(map)
    }

    fn lookup(&self, key: &Symbol) -> Option<CompileTimeBinding> {
        self.0.get(key).cloned()
    }
}

// TODO: return error if ambiguous
// or maybe return error in resolve, instead of option
fn check_unambiguous<'a>(id: &Syntax, mut candidate_ids: impl Iterator<Item = &'a Syntax>) -> bool {
    candidate_ids.all(|c_id| c_id.1.is_subset(&id.1))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{AdjustScope, Ast, Binding, Expander, Scope, Symbol, Syntax, UniqueNumberManager};

    #[test]
    fn identifier_test_with_empty_syntax() {
        assert!(Ast::Syntax(Syntax::new("a".into())).identifier());
    }

    #[test]
    fn datum_to_syntax_with_identifier() {
        assert_eq!(
            Ast::Symbol(Symbol("a".into(), 0)).datum_to_syntax(),
            Ast::Syntax(Syntax("a".into(), BTreeSet::new()))
        );
    }

    #[test]
    fn datum_to_syntax_with_number() {
        assert_eq!(Ast::Number(1.0).datum_to_syntax(), Ast::Number(1.0));
    }

    #[test]
    fn datum_to_syntax_with_list() {
        assert_eq!(
            Ast::List(vec![
                Ast::Symbol(Symbol("a".into(), 0)),
                Ast::Symbol(Symbol("b".into(), 0)),
                Ast::Symbol(Symbol("c".into(), 0)),
            ])
            .datum_to_syntax(),
            Ast::List(vec![
                Ast::Syntax(Syntax("a".into(), BTreeSet::new())),
                Ast::Syntax(Syntax("b".into(), BTreeSet::new())),
                Ast::Syntax(Syntax("c".into(), BTreeSet::new())),
            ])
        );
    }

    #[test]
    fn datum_to_syntax_with_list_and_syntax() {
        assert_eq!(
            Ast::List(vec![
                Ast::Symbol(Symbol("a".into(), 0)),
                Ast::Syntax(Syntax("b".into(), BTreeSet::from([Scope(0), Scope(1)]))),
                Ast::Symbol(Symbol("c".into(), 0)),
            ])
            .datum_to_syntax(),
            Ast::List(vec![
                Ast::Syntax(Syntax("a".into(), BTreeSet::new())),
                Ast::Syntax(Syntax("b".into(), BTreeSet::from([Scope(0), Scope(1)]))),
                Ast::Syntax(Syntax("c".into(), BTreeSet::new())),
            ])
        );
    }
    #[test]
    fn syntax_to_datum_with_identifier() {
        assert_eq!(
            Ast::Syntax(Syntax("a".into(), BTreeSet::new())).syntax_to_datum(),
            Ast::Symbol(Symbol("a".into(), 0)),
        );
    }

    #[test]
    fn syntax_to_datum_with_number() {
        assert_eq!(Ast::Number(1.0).syntax_to_datum(), Ast::Number(1.0));
    }

    #[test]
    fn syntax_to_datum_with_list() {
        assert_eq!(
            Ast::List(vec![
                Ast::Syntax(Syntax("a".into(), BTreeSet::new())),
                Ast::Syntax(Syntax("b".into(), BTreeSet::new())),
                Ast::Syntax(Syntax("c".into(), BTreeSet::new())),
            ])
            .syntax_to_datum(),
            Ast::List(vec![
                Ast::Symbol(Symbol("a".into(), 0)),
                Ast::Symbol(Symbol("b".into(), 0)),
                Ast::Symbol(Symbol("c".into(), 0)),
            ])
        );
    }

    #[test]
    fn scope_equality_test() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        let sc2 = scope_creator.new_scope();
        assert_ne!(sc1, sc2);
        assert_eq!(sc2, sc2);
    }

    #[test]
    fn add_scope_test_empty_scope() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        assert_eq!(
            Ast::Syntax(Syntax("x".into(), BTreeSet::new())).add_scope(sc1),
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1])))
        );
    }

    #[test]
    fn add_scope_test_empty_scope_list() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        assert_eq!(
            Ast::List(vec![
                Ast::Symbol(Symbol("x".into(), 0)),
                Ast::Symbol(Symbol("y".into(), 0)),
            ])
            .datum_to_syntax()
            .add_scope(sc1),
            Ast::List(vec![
                Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1]))),
                Ast::Syntax(Syntax("y".into(), BTreeSet::from([sc1]))),
            ])
        );
    }

    #[test]
    fn add_scope_test_non_empty_scope() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        let sc2 = scope_creator.new_scope();
        assert_eq!(
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1]))).add_scope(sc2),
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1, sc2])))
        );
    }

    #[test]
    fn add_scope_test_add_duplicate() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        assert_eq!(
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1]))).add_scope(sc1),
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1,])))
        );
    }

    #[test]
    fn flip_scope_test_different() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        let sc2 = scope_creator.new_scope();
        assert_eq!(
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1]))).flip_scope(sc2),
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1, sc2])))
        );
    }

    #[test]
    fn flip_scope_test_same() {
        let mut scope_creator = UniqueNumberManager::new();
        let sc1 = scope_creator.new_scope();
        let sc2 = scope_creator.new_scope();
        assert_eq!(
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1, sc2]))).flip_scope(sc2),
            Ast::Syntax(Syntax("x".into(), BTreeSet::from([sc1,])))
        );
    }

    #[test]
    fn resolve_test_lambda_empty() {
        let expander = Expander::new();

        assert_eq!(
            expander.resolve(&Syntax("lambda".into(), BTreeSet::new())),
            None
        );
    }

    #[test]
    fn resolve_test_lambda_core() {
        let expander = Expander::new();

        println!("{expander:?}");
        assert_eq!(
            expander.resolve(&expander.introduce(Syntax("lambda".into(), BTreeSet::new()))),
            Some(Binding::Lambda).as_ref()
        );
    }
}
