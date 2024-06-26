//! code for macro parsing, expansion, and primitive macros

// TODO: create macro syntax
// TODO: quotation, list manipulation, ...
// TODO: define primitive macros for cond, link, let, ... (and maybe special way for primitive macros)
// primitive macro link needs some additional context to save what labels are linked to other labels which then gets passed to the compiler to make global goto possible

// Macros have their own phase called macro expansion
// it is kind of like an evaluation but macros can only currently be defined in the global scope
// macros use quotation and list manipulation to tranform syntax making it easier to create syntatic abstraction
// some implementations will recursively expand macros, like when a user-defined macro calls a primitive macro
// for now this will be a non-hygenic macro expander

// TODO more builtins an cleanup better error handlin,g types, andrmesages
use crate::lexer::Error;
use std::collections::HashMap;

use itertools::Itertools;

use crate::{ast::ast1::Ast1, interior_mut::RC, lexer::everything_parse};
use std::fs;
use std::io::BufReader;
use std::io::Read;
#[derive(Debug)]
pub enum ParseError<'a, E> {
    Macro(MacroError),
    Parse(super::pc::ParseError<'a, E>),
}

pub fn parse_and_expand(
    input: &str,
) -> Result<(Vec<Ast1>, HashMap<RC<str>, Vec<RC<str>>>), ParseError<'_, Error>> {
    let ast = everything_parse(input).map_err(ParseError::Parse)?;
    let mut expander = MacroExpander::new();
    let expanded = expander.expand_get_link(&ast).map_err(ParseError::Macro)?;
    Ok(expanded)
}

trait HashMapExtend {
    fn push_nested(&mut self, nested: Self) -> Option<()>;
    fn merge(&mut self, other: Self);
    fn get(&self, key: &RC<str>) -> Result<&Ast1, MacroExpansionError>;
}

impl HashMapExtend for HashMap<RC<str>, MacroBinding> {
    fn push_nested(&mut self, nested: Self) -> Option<()> {
        for (k, v) in nested {
            if !self.contains_key(&k) {
                self.insert(k.clone(), MacroBinding::List(vec![]));
            }
            match self.get_mut(&k) {
                Some(MacroBinding::List(l)) => l.push(v),
                _ => return None,
            }
        }
        Some(())
    }

    fn merge(&mut self, other: Self) {
        // rn just just extend but might not be that simple
        self.extend(other);
    }

    fn get(&self, key: &RC<str>) -> Result<&Ast1, MacroExpansionError> {
        let b = self
            .get(key)
            .ok_or_else(|| MacroExpansionError::MetaVariableNotFound(key.clone()))?;

        match b {
            MacroBinding::Expr(it) => Ok(it),
            MacroBinding::List(_) => Err(MacroExpansionError::DepthAndVariableDepthNoMatch),
            MacroBinding::Unbound => Err(MacroExpansionError::ReptitionAndVarialbeNotMatch),
        }
    }
}

#[derive(Debug)]
pub enum MacroError {
    InvalidForm(RC<str>),
    InvalidMacroCase,
    NoMacroCases,
    InvalidMacroCondition,
    NoMacroForms,
    MacroExpansion(MacroExpansionError),
    CaseMismatch,
    NoCaseMatches,
}

#[derive(Debug, Clone)]
pub enum MacroExpansionError {
    MetaVariableNotFound(RC<str>),
    KleeneClosureWithoutNestedBindings,
    DepthAndVariableDepthNoMatch,
    ReptitionAndVarialbeNotMatch,
}

#[derive(Default)]
pub struct MacroExpander {
    // TODO: make macro type
    macro_env: HashMap<RC<str>, MacroType>,
    links: HashMap<RC<str>, Vec<RC<str>>>,
}

impl MacroExpander {
    pub fn new() -> Self {
        let mut this = Self::default();
        this.macro_env.insert(
            "link".into(),
            MacroType::SpecialForm(Self::special_form_link),
        );
        this.macro_env.insert(
            "defmacro".into(),
            MacroType::SpecialForm(Self::special_form_macro),
        );
        this.macro_env.insert(
            "cond".into(),
            MacroType::SpecialForm(Self::special_form_cond),
        );
        this.macro_env.insert(
            "module".into(),
            MacroType::SpecialForm(Self::special_form_module),
        );
        this
    }

    /// expands the expressions provided and returns the expanded expressions and the links for goto expressions
    /// it clears the links from the expander
    pub fn expand_get_link(
        &mut self,
        exprs: &[Ast1],
    ) -> Result<(Vec<Ast1>, HashMap<RC<str>, Vec<RC<str>>>), MacroError> {
        self.expand(exprs)
            .map(|res| (res, std::mem::take(&mut self.links)))
    }

    fn expand(&mut self, exprs: &[Ast1]) -> Result<Vec<Ast1>, MacroError> {
        let mut res = vec![];
        for expr in exprs {
            match expr {
                Ast1::Application(a) => {
                    // we expand all the sub expressions of the application before expanding the application itself
                    // which most macro expandets dont do so we should probably expand outer than inner if possible
                    // TODO: we aslo need recurasve macro epxansiion working ie try to expand list if not possbile try to expandsubexpression ....
                    let a = self.expand(a)?;
                    if let Some(Ast1::Ident(op)) = a.first() {
                        if let Some(r#macro) = self.macro_env.get(op) {
                            match r#macro {
                                MacroType::SpecialForm(sf) => {
                                    res.extend(sf(self, &a[1..])?);
                                }
                                MacroType::UserDefined(cases) => {
                                    let expander = cases
                                        .iter()
                                        .find_map(|(case, res)| {
                                            // case.
                                            let MacroArg::List(case) = case else {
                                                unreachable!()
                                            };
                                            matches(&a[1..], case)
                                                .map(|bindings| (bindings, res))
                                                .ok()
                                        })
                                        .ok_or(MacroError::NoCaseMatches)?;
                                    res.extend(
                                        expand_macro(&expander.0, expander.1.clone())
                                            .map_err(MacroError::MacroExpansion)?,
                                    );
                                }
                            }
                        } else {
                            res.push(expr.clone());
                        }
                    } else {
                        res.push(expr.clone());
                    }
                }
                _ => {
                    res.push(expr.clone());
                }
            }
        }
        Ok(res)
    }
}
fn expand_macro(
    bindings: &HashMap<RC<str>, MacroBinding>,
    expansion: Vec<Ast1>,
) -> Result<Vec<Ast1>, MacroExpansionError> {
    fn expand_macro_inner(
        bindings: &HashMap<RC<str>, MacroBinding>,
        expansion: Ast1,
    ) -> Result<Ast1, MacroExpansionError> {
        match expansion {
            Ast1::Application(a) => expand_macro(bindings, a).map(Ast1::Application),
            Ast1::Ident(i) => match HashMapExtend::get(bindings, &i) {
                Ok(res) => Ok(res.clone()),
                Err(MacroExpansionError::MetaVariableNotFound(_)) => Ok(Ast1::Ident(i)),
                Err(e) => Err(e),
            },
            other => Ok(other),
        }
    }
    let mut res = Vec::new();
    let mut expansion = expansion.into_iter().peekable();
    while let Some(expander) = expansion.next() {
        // TODO: handle more than one kleene expansion ie (a * *)
        if expansion.peek() == Some(&Ast1::Ident("*".into())) {
            expansion.next(); // swallow kleene closure

            let mut expander = vec![expander];
            while expansion.peek() == Some(&Ast1::Ident("*".into())) {
                expander.push(expansion.next().unwrap());
            }
            let expander = expander;

            // instead we need to know how many expressions connected to the idents in expansion and loop that many times
            // after each kleene closure is found remove a level for each binding which is a list
            // to make this work in reality after we hit kleene closure we check if there are any bindings that are lists otherwise error
            // we then iterate n times where n is the length of the list of a binding such that the list is the longest of all the binding lists
            // during each iteration we create new bindings by going through the binding and
            // if binding is expression in new bindings binding will same expression
            // if list if nth element of binding is present then insert nth element of binding as binding in new bindings binding otherwise do not bind in new bindings binding

            // still has problems when were at a depth in expansion where there is kleene closure but the binding for kleene closure is not one being used
            // like             "(defmacro test [((c * b ) * a) (display b ) * *  ])"
            let largest = bindings
                .iter()
                .filter_map(|binding| {
                    if let MacroBinding::List(l) = binding.1 {
                        Some(l.len())
                    } else {
                        None
                    }
                })
                .max()
                .ok_or(MacroExpansionError::KleeneClosureWithoutNestedBindings)?;

            let (errs, oks): (Vec<_>, Vec<_>) = (0..largest)
                .map(move |n| {
                    let bindings: HashMap<_, _> = bindings
                        .clone()
                        .into_iter()
                        .map(|binding| {
                            if let MacroBinding::List(l) = binding.1 {
                                (
                                    binding.0,
                                    l.get(n).unwrap_or(&MacroBinding::Unbound).clone(),
                                )
                            } else {
                                binding
                            }
                        })
                        .collect();
                    expand_macro(&bindings, expander.clone())
                    // filter out error that are do to tring to larget amo=ount of time but the meta variable used was not the largest one
                })
                .filter(|expansion| {
                    !matches!(
                        expansion,
                        Err(MacroExpansionError::ReptitionAndVarialbeNotMatch)
                    )
                })
                .partition_map(From::from);
            if oks.is_empty() {
                // if there is no error means no expansionie nothing went wrong
                if !errs.is_empty() {
                    return Err(errs[0].clone());
                }
            }
            res.extend(oks.into_iter().flatten());
        } else {
            res.push(expand_macro_inner(bindings, expander)?);
        }
    }

    Ok(res)
}

// special forms
impl MacroExpander {
    fn special_form_module(&mut self, exprs: &[Ast1]) -> Result<Vec<Ast1>, MacroError> {
        let mut body = if exprs.len() == 1 && matches!(exprs[0], Ast1::String(_)) {
            let Ast1::String(path) = &exprs[0] else {
                unreachable!()
            };
            // it must be module as a path
            // TODO: dont panic but error
            let file = fs::File::open(path.to_string()).unwrap();

            let mut buf = BufReader::new(file);
            let mut contents = String::new();
            buf.read_to_string(&mut contents);
            everything_parse(&contents).unwrap()
        } else {
            exprs.to_vec()
        };
        body = self.expand(&body)?;
        body.insert(0, "mod".into());
        Ok(body)
    }
    // TODO: module import special form - class special form
    fn special_form_link(&mut self, exprs: &[Ast1]) -> Result<Vec<Ast1>, MacroError> {
        let Some(Ast1::Label(linked)) = exprs.get(0) else {
            return Err(MacroError::InvalidForm(
                "first expr of link must be a label".into(),
            ));
        };
        let links = if let Some(links) = exprs.get(1..) {
            links
                .iter()
                .map(|expr| match expr {
                    Ast1::Label(l) => Some(l.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<RC<str>>>>()
                .ok_or_else(|| {
                    MacroError::InvalidForm("the elements of the link are not all labels".into())
                })?
        } else {
            return Err(MacroError::InvalidForm(
                "there must at least one other expr in a link".into(),
            ));
        };
        self.links.insert(linked.clone(), links);
        Ok(vec![])
    }

    fn special_form_macro(&mut self, exprs: &[Ast1]) -> Result<Vec<Ast1>, MacroError> {
        let Some(Ast1::Ident(macro_name)) = exprs.get(0) else {
            return Err(MacroError::InvalidForm(
                "defmacro expects a name for the macro".into(),
            ));
        };
        if let Some(cases) = exprs.get(1..) {
            let cases = MacroType::UserDefined(
                cases
                    .iter()
                    .map(|case| {
                        let Ast1::Application(rule) = case else {
                            return Err(MacroError::InvalidMacroCase);
                        };

                        let Some(Ast1::Application(case)) = rule.first() else {
                            return Err(MacroError::InvalidMacroCase);
                        };
                        let cond: MacroArg = case.as_slice().try_into()?;

                        let expand = rule.get(1..).ok_or(MacroError::NoMacroForms)?.to_vec();
                        Ok((cond, expand))
                        // TODO: parse the expansion part
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if self.macro_env.insert(macro_name.clone(), cases).is_some() {
                // Error redefiniton of macro
            }
            Ok(vec![])
        } else {
            Err(MacroError::NoMacroCases)
        }
    }

    fn special_form_cond(_: &mut Self, exprs: &[Ast1]) -> Result<Vec<Ast1>, MacroError> {
        fn cond_expand(exprs: &[Ast1]) -> Result<Ast1, MacroError> {
            if let Some(case) = exprs.first() {
                if let Ast1::Application(case) = case {
                    let mut case = case.clone();
                    match case.first() {
                        Some(Ast1::Ident(e)) if (e.clone()) == "else".into() => {
                            if exprs.get(1).is_some() {
                                Err(MacroError::InvalidForm("else with cases after it".into()))
                            } else {
                                Ok(Ast1::Application({
                                    case[0] = "begin".into();
                                    case
                                }))
                            }
                        }
                        Some(expr) => Ok(Ast1::Application(vec![
                            "if".into(),
                            expr.clone(),
                            Ast1::Application({
                                case[0] = "begin".into();
                                case
                            }),
                            cond_expand(&exprs[1..])?,
                        ])),
                        None => {
                            // Error empty case
                            Err(MacroError::InvalidForm("macro cond is empty".into()))
                        }
                    }
                } else {
                    // Error case not list
                    Err(MacroError::InvalidForm("macro case is not a list".into()))
                }
            } else {
                Ok(Ast1::Bool(crate::ast::Boolean::False))
            }
        }
        Ok(vec![cond_expand(exprs)?])
    }
}

pub enum MacroType {
    SpecialForm(fn(&mut MacroExpander, &[Ast1]) -> Result<Vec<Ast1>, MacroError>),
    // the parameters of a macro can nested lists and can have constraints such taht certain symbols are required
    UserDefined(Vec<(MacroArg, Vec<Ast1>)>),
}

#[derive(Debug)]
pub enum MacroArg {
    // a list of macro arguments
    List(Vec<MacroArg>),
    Ident(RC<str>),
    // when encountred during macro expansion must match specifically this symbol
    Constant(RC<str>),
    // will be some if there is a arg before it
    KleeneClosure(Option<Box<MacroArg>>),
}

impl TryFrom<&[Ast1]> for MacroArg {
    type Error = MacroError;

    fn try_from(value: &[Ast1]) -> Result<Self, Self::Error> {
        // TOOD: check for empty kleene closures if they have something before them
        // and if more than one kleene closure error
        let mut ret: Vec<_> = value
            .iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;
        let mut temp = Self::KleeneClosure(None);
        if let Some(Self::KleeneClosure(_)) = ret.first() {
            if ret
                .iter()
                .skip(1)
                .any(|arg| matches!(&arg, Self::KleeneClosure(_)))
            {
                Err(MacroError::InvalidMacroCondition)
            } else {
                Ok(Self::List(ret))
            }
        } else if let Some(pos) = ret
            .iter()
            .position(|arg| matches!(arg, Self::KleeneClosure(_)))
        {
            if ret
                .iter()
                .skip(pos + 1)
                .any(|arg| matches!(&arg, Self::KleeneClosure(_)))
            {
                // if we encounter 2 *
                Err(MacroError::InvalidMacroCondition)
            } else {
                // using indexing is ok here (no panic) b/c we alreadynkow the index is valid from postion
                std::mem::swap(&mut ret[pos - 1], &mut temp);
                ret[pos - 1] = Self::KleeneClosure(Some(Box::new(temp)));
                ret.remove(pos);
                Ok(Self::List(ret))
            }
        } else {
            Ok(Self::List(ret))
        }
    }
}

impl TryFrom<&Ast1> for MacroArg {
    type Error = MacroError;

    fn try_from(value: &Ast1) -> Result<Self, Self::Error> {
        match value {
            Ast1::Application(a) => a.as_slice().try_into(),
            Ast1::Ident(i) => Ok(if i == &("*".into()) {
                Self::KleeneClosure(None)
            } else {
                Self::Ident(i.clone())
            }),
            Ast1::String(s) => Ok(Self::Constant(s.clone())),
            _ => todo!("error"),
        }
    }
}
//https://github.com/rust-lang/rust-analyzer/pull/719/files#diff-ebf19883d233fae7be3aec83af917278e2ed7b79d3847c69941dfdf0728b0583 (could be helpfull)
#[derive(Debug, Clone)]
pub enum MacroBinding {
    // generated from matching *
    List(Vec<MacroBinding>),
    Expr(Ast1),
    Unbound,
}

#[derive(Debug)]
pub enum MacroMatchError {}

impl MacroArg {
    fn matches(&self, pattern: &Ast1) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
        let mut bindings = HashMap::new();
        // TODO: i think a lot/most of the todo cases are unreachable
        match (self, pattern) {
            (Self::List(pattern), Ast1::Application(expr)) => {
                bindings.merge(matches(expr, pattern)?);
            }
            // error
            (Self::List(_), _) => todo!(),
            (Self::Ident(i), expr) => {
                bindings.insert(i.clone(), MacroBinding::Expr(expr.clone()));
            }

            (Self::Constant(a), Ast1::Ident(b)) if a == b => todo!(),
            // error
            (Self::Constant(_), _) => todo!(),
            (Self::KleeneClosure(_), Ast1::Bool(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::Number(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::String(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::Ident(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::Application(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::Label(_)) => todo!(),
            (Self::KleeneClosure(_), Ast1::FnParam(_)) => todo!(),
        }
        Ok(bindings)
    }

    // used for kleen closure matching so that even if kleen clssore matches 0 times there will be actual bindings for each metavariable
    fn init_bindings(&self) -> HashMap<RC<str>, MacroBinding> {
        match self {
            Self::List(l) => l.iter().flat_map(Self::init_bindings).collect(),
            Self::Ident(i) => HashMap::from([(i.clone(), MacroBinding::List(vec![]))]),
            Self::Constant(_) | Self::KleeneClosure(None) => HashMap::new(),
            Self::KleeneClosure(Some(arg)) => {
                let mut res = HashMap::new();
                res.push_nested(arg.init_bindings());
                res
            }
        }
    }
}

fn matches(
    expr: &[Ast1],
    pattern: &[MacroArg],
) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
    let mut expr_count = expr.len();
    let mut pat_count = pattern.len();
    let mut bindings = HashMap::new();
    let mut expr = expr.iter();
    for pat in pattern {
        match pat {
            MacroArg::List(pat) => {
                let Some(Ast1::Application(expr)) = expr.next() else {
                    return Err(MacroError::CaseMismatch);
                };
                bindings.merge(matches(expr, pat)?);
                expr_count -= 1;
            }
            MacroArg::Ident(i) => {
                let expr = expr.next().ok_or(MacroError::CaseMismatch)?;
                bindings.insert(i.clone(), MacroBinding::Expr(expr.clone()));
                expr_count -= 1;
            }
            MacroArg::Constant(c) => {
                if Some(&Ast1::Ident(c.clone())) != expr.next() {
                    return Err(MacroError::CaseMismatch);
                }
                expr_count -= 1;
            }
            MacroArg::KleeneClosure(c) => {
                let taken_count = expr_count.checked_sub(pat_count - 1).unwrap_or(expr_count);
                expr_count -= taken_count;
                let matched = expr.clone().take(taken_count);

                expr.nth(taken_count - 1);

                if let Some(c) = c {
                    bindings.merge(c.init_bindings());
                    for expr in matched {
                        bindings
                            .push_nested(c.matches(expr)?)
                            .ok_or(MacroError::CaseMismatch)?;
                    }
                }
            }
        }
        pat_count -= 1;
    }
    if expr.len() != 0 {
        return Err(MacroError::CaseMismatch);
    };
    Ok(bindings)
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast1::Ast1, lexer::everything_parse};

    use super::MacroExpander;

    #[test]
    fn basic_macro_test() {
        let parsed = everything_parse(
            "(defmacro test [(a) (a a)])
        (test 1)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![Ast1::Application(vec![
                Ast1::Number(1.0),
                Ast1::Number(1.0)
            ])]
        );
    }

    #[test]
    fn basic_macro_constant_test() {
        let parsed = everything_parse(
            "(defmacro test [(.a.) (1 1)])
        (test a)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![Ast1::Application(vec![
                Ast1::Number(1.0),
                Ast1::Number(1.0)
            ])]
        );
    }

    #[test]
    fn basic_macro_kleene_test() {
        let parsed = everything_parse(
            "(defmacro test [(b * a) (display b) *])
        (test 1 2 3 4)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(1.0)]),
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(2.0)]),
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(3.0)]),
            ]
        );
    }

    #[test]
    fn basic_macro_kleene_list_test() {
        let parsed = everything_parse(
            "(defmacro test [((c * b ) * a) (display c ) * *  ])
        (test (1 4 5) (4 7 8 ) (4 7 7 ) 6) ",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(5.0)]),
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(8.0)]),
                Ast1::Application(vec![Ast1::Ident("display".into()), Ast1::Number(7.0)]),
            ]
        );
    }

    #[test]
    fn basic_macro_kleene_ignore_test() {
        let parsed = everything_parse(
            "(defmacro test [(* a) (display a)])
        (test (1 4 5) (4 7 8 ) (4 7 8 ) 6)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![Ast1::Application(vec![
                Ast1::Ident("display".into()),
                Ast1::Number(6.0)
            ]),]
        );
    }

    #[test]
    fn complexish_macro_kleene_test() {
        let parsed = everything_parse(
            "(defmacro test [((c .ret. b * a ) * q) (display a ) *])
        (test (1 ret 4 y v 5) (4 ret 7 u  8 ) (4 ret 7 8 )  1)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![Ast1::Application(vec![
                Ast1::Ident("display".into()),
                Ast1::Number(6.0)
            ]),]
        );
    }
}
