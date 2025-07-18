use crate::{
    ast::{ast1::Ast1, Ast, Boolean, Function, Lambda, Pair, Param, Symbol},
    error::{Error, ExpectedSingleValue},
    expander::expand_expr::list_to_cons,
    matches_to,
    primitives::new_primitive_env,
};

use clap::error::Result;
use itertools::Itertools;
use rand::random;

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Clone, PartialEq, Debug)]
pub struct Env {
    scope: HashMap<Symbol, Ast>,
    parent: Option<EnvRef>,
}

impl Env {
    pub fn new_env() -> Rc<RefCell<Self>> {
        let env = Self::new();
        new_primitive_env(|name, primitive| {
            env.borrow_mut().define(name.into(), primitive);
        });
        env
    }
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
    fn define_values(
        &mut self,
        symbols: impl IntoIterator<Item = Symbol>,
        values: impl IntoIterator<Item = Ast>,
    ) {
        self.scope.extend(symbols.into_iter().zip(values));
    }
    pub fn new_scope(env: EnvRef) -> EnvRef {
        let parent = Some(env);
        let scope = HashMap::new();
        Rc::new(RefCell::new(Self { scope, parent }))
    }

    pub fn new_lambda_env(env: EnvRef, symbol: Symbol, expr: Ast) -> EnvRef {
        let new_env = Self::new_scope(env);
        new_env.borrow_mut().define(symbol, expr);
        new_env
    }
    // pub fn extend_envoirnment(env: EnvRef, params: Ast, args: Ast) -> Result<EnvRef, Error> {
    //     let new_envoirnment = Self::new_scope(env);
    //     fn extend_envoirnment(env: Rc<RefCell<Env>>, params: Ast, args: Ast) -> Result<(), Error> {
    //         match (params, args) {
    //             (Ast::Pair(param), Ast::Pair(arg)) => {
    //                 let Ast::Symbol(p) = param.0 else {
    //                     return Err(format!(
    //                         "{} is not a symbol (cannot be function arguement)",
    //                         param.1
    //                     ));
    //                 };
    //                 env.borrow_mut().define(p, arg.0);
    //                 extend_envoirnment(env, param.1, arg.1)
    //             }
    //             (Ast::Symbol(s), rest) => {
    //                 env.borrow_mut().define(s, rest);
    //                 Ok(())
    //             }
    //             (Ast::TheEmptyList, Ast::TheEmptyList) => Ok(()),
    //             (Ast::TheEmptyList, rest) => Err(format!(
    //                 "arity mismatch these arguments do not have any coresponding parameters {rest}"
    //             )),
    //             (rest, Ast::TheEmptyList) => Err(format!(
    //                 "arity mismatch these parameters do not have any coresponding aruments {rest}"
    //             )),
    //             (arg, _) => Err(format!("bad argument expected symbol found: {arg}")),
    //         }
    //     }
    //     extend_envoirnment(new_envoirnment.clone(), params.clone(), args.clone())
    //         .map(|()| new_envoirnment)
    //         .map_err(|e| format!("{e} {params} {args}"))
    // }

    pub(crate) fn new() -> EnvRef {
        let scope = HashMap::new();
        // TODO: primitive environment
        let parent = None;
        Rc::new(RefCell::new(Self { scope, parent }))
    }
}
pub type EnvRef = Rc<RefCell<Env>>;

pub struct Evaluator {}

#[derive(Debug)]
pub enum Values {
    Many(Vec<Ast>),
    Single(Ast),
}

impl PartialEq for Values {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Many(l0), Self::Many(r0)) => l0 == r0,
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Single(l0), Self::Many(r0)) if r0.len() == 1 => {
                r0.first().is_some_and(|r0| r0 == l0)
            }
            (Self::Many(l0), Self::Single(r0)) if l0.len() == 1 => {
                l0.first().is_some_and(|l0| l0 == r0)
            }
            _ => false,
        }
    }
}
impl Values {
    pub fn into_single(self) -> Result<Ast, Vec<Ast>> {
        match self {
            Self::Many(mut vec) if vec.len() == 1 => {
                let ast = vec.remove(0);
                Ok(ast)
            }
            Self::Many(vec) => Err(vec),
            Self::Single(ast) => Ok(ast),
        }
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Many(vec) => write!(
                f,
                "{}",
                vec.iter().map(ToString::to_string).collect_vec().join("\n")
            ),
            Self::Single(ast) => write!(f, "{ast}"),
        }
    }
}
fn parse_formals(value: &Ast) -> Result<Param, Error> {
    match value {
        Ast::TheEmptyList => Ok(Param::Zero),
        Ast::Pair(ref arg_list) => {
            let Pair(ref argc, ref rest) = **arg_list;
            let argc = matches_to!(argc => Ast::Symbol).ok_or("argc must be a number")?;
            match rest {
                Ast::TheEmptyList => Ok(Param::One(argc.0.clone())),
                Ast::Pair(ref varidiac) => match **varidiac {
                    Pair(Ast::Symbol(ref variadic), Ast::TheEmptyList)
                        if variadic.to_string() == "*" =>
                    {
                        Ok(Param::AtLeast0(argc.0.clone()))
                    }
                    Pair(Ast::Symbol(ref variadic), Ast::TheEmptyList)
                        if variadic.to_string() == "+" =>
                    {
                        Ok(Param::AtLeast1(argc.0.clone()))
                    }
                    _ => Err(format!("parameter must be a list {value}").into()),
                },
                _ => Err(format!("parameter must be a list {value}").into()),
            }
        }
        _ => Err(format!("parameter must be a list {value}").into()),
    }
}
impl Evaluator {
    pub(crate) fn eval_single_value(expr: Ast1, env: EnvRef) -> Result<Ast, Error> {
        Self::eval(expr, env).and_then(|values| {
            values
                .into_single()
                .map_err(|_| Error::ExpectedSingleValue(ExpectedSingleValue()))
        })
    }
    pub(crate) fn eval(expr: Ast1, env: EnvRef) -> Result<Values, Error> {
        match expr {
            Ast1::Lambda(param, body) => {
                Ok(Values::Single(Ast::Function(Function::Lambda(Lambda {
                    body,
                    env,
                    param,
                }))))
            }
            Ast1::Quote(datum) => Ok(Values::Single(datum)),
            Ast1::Begin(b) => Self::eval_sequence(b, env),
            Ast1::Begin0(b) => todo!(),
            Ast1::LetRecValues(v, b) => {
                v.into_iter().try_for_each(|(mut ids, value)| -> Result<_, Error> {
                                        let value = Self::eval(value, Rc::clone(&env))?;
                                        match value {
                                            Values::Many(vec) if vec.len() == ids.len() => {
                                                env.borrow_mut().define_values(ids.into_iter().map(Symbol), vec);
                                                Ok(())
                                            },
                                            Values::Single(ast) if ids.len() == 1 => {
                                                env.borrow_mut().define(Symbol(ids.remove(0)), ast);
                                                Ok(())
                                            },
                                            _ => Err("let-values error: number of values is not the same as the number of ids".into()),
                                        }
                                    })?;
                Self::eval(*b, env)
            }
            Ast1::LetValues(v, b) => {
                let values = v.into_iter().map(|(mut ids, value)|-> Result<_, Error> {
                                        let value = Self::eval(value, Rc::clone(&env))?;
                                        match value {
                                            Values::Many(vec) if vec.len() == ids.len() => {
                                                Ok(ids.into_iter().map(Symbol).zip(vec).collect_vec())
                                            },
                                            Values::Single(ast) if ids.len() == 1 => Ok(vec![(Symbol(ids.remove(0)), ast)]),
                                            _ => Err("let-values error: number of values is not the same as the number of ids".into()),
                                        }
                                    }).try_collect::<_, Vec<_>, _>()?.concat();
                let env = Rc::new(RefCell::new(Env {
                    scope: HashMap::from_iter(values),
                    parent: Some(env),
                }));
                Self::eval(*b, env)
            }
            Ast1::Application(f, args) => {
                let f: Ast = Self::eval_single_value(*f, env.clone())?;
                let rest = list_to_cons(
                    args.into_iter()
                        .map(|arg| Self::eval_single_value(arg, env.clone()))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter(),
                    |x| x,
                );

                Self::execute_application(f, rest)
            }
            Ast1::Basic(Ast::Symbol(s)) => env
                .borrow()
                .lookup(&s)
                .map(Values::Single)
                .ok_or(format!("free variable {s} eval").into()),
            Ast1::Basic(expr) => Ok(Values::Single(expr)),
            Ast1::If(ast1, ast2, ast3) => {
                let cond = Self::eval_single_value(*ast1, env.clone())?;
                let cond = matches!(cond, Ast::Boolean(Boolean::False | Boolean::Maybe) if random::<u8>() % 2 == 0);
                if cond {
                    Self::eval(*ast2, env)
                } else {
                    Self::eval(*ast3, env)
                }
            }
            Ast1::DefineValues(mut ids, value) => {
                let value = Self::eval(*value, env.clone())?;
                match value {
                    Values::Many(vec) if vec.len() == ids.len() => {
                        env.borrow_mut().define_values(ids.into_iter().map(Symbol), vec);
                        Ok(Values::Many(vec![]))
                    }
                    Values::Single(ast) if ids.len() == 1 => {
                        env.borrow_mut().define(Symbol(ids.remove(0)),  ast);
                        Ok(Values::Many(vec![]))
                    },
                    _ => Err(
                        "define-values error: number of values is not the same as the number of ids"
                            .into(),
                    ),
                }
            }
            Ast1::Set(s, ast1) => {
                let value = Self::eval_single_value(*ast1, env.clone())?;
                env.borrow_mut().set(Symbol(s), value);
                Ok(Values::Many(vec![]))
            }
            Ast1::Stop(ast1) => todo!(),
            Ast1::Skip => todo!(),
            Ast1::Loop(ast1) => todo!(),
            Ast1::Module(_, module_type) => todo!(),
            Ast1::Expression(ast1) => Self::eval(*ast1, env),
        }
    }

    pub(crate) fn execute_application(f: Ast, args: Ast) -> Result<Values, Error> {
        if let Ast::Function(f) = f {
            f.apply(args)
        } else {
            Err(format!("cannot not apply {f} to {args:?}, because {f} is not a function").into())
        }
    }

    pub(crate) fn eval_sequence(body: Vec<Ast1>, env: Rc<RefCell<Env>>) -> Result<Values, Error> {
        // this might not fail if anything but last one doesn't fail
        body.into_iter()
            .map(|v| Self::eval(v, env.clone()))
            .next_back()
            .ok_or("empty begin".to_string())?
    }
    pub fn to_id_list(ids: Ast) -> Result<Vec<Symbol>, Error> {
        let ids = ids.to_list_checked::<Error>()?;
        let ids = ids
            .into_iter()
            .map(std::convert::TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ids)
    }
    fn check_let(letrec_values: &str, list: Ast) -> Result<(Vec<(Vec<Symbol>, Ast)>, Ast), Error> {
        let Ast::Pair(pair) = list else {
            return Err(format!("error {letrec_values}: expected key values pairs").into());
        };
        let values = pair.0;
        let values: Vec<(Vec<Symbol>, Ast)> = values
            .to_list_checked::<Error>()?
            .into_iter()
            .map(|value| -> Result<_, Error> {
                let Ast::Pair(value) = value else {
                    return Err(format!(
                        "error {letrec_values}: expected [(ids) value], got {value}"
                    )
                    .into());
                };
                let ids = Self::to_id_list(value.0)?;
                let Ast::Pair(value) = value.1 else {
                    return Err(format!(
                        "error {letrec_values}: expected [(ids) value], missing value {}",
                        value.1
                    )
                    .into());
                };
                if value.1 != Ast::TheEmptyList {
                    return Err(format!(
                        "error {letrec_values}: expected [(ids) value], it is unclosed, got {}",
                        value.1
                    )
                    .into());
                }
                Ok((ids, value.0))
            })
            .try_collect()?;
        let Ast::Pair(pair) = pair.1 else {
            return Err(format!("error {letrec_values}: expected body").into());
        };
        let body = pair.0;
        if pair.1 != Ast::TheEmptyList {
            return Err(format!("error {letrec_values}: unclosed body").into());
        }
        Ok((values, body))
    }
}
