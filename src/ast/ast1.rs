use std::{collections::HashMap, fmt, hash::Hash};

use itertools::Itertools;

use crate::interior_mut::{MUTEX, RC};

use super::{Ast, ModuleType, Param};

#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(Ast1, Ast1, Ast1)>>,
}

// TODO: flatten trait for quotation
// pub trait FlattenAst<'a, 'ctx> {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx>;
// }

// impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Ast1 {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
//         match self {
//             Self::Boolean(b) => compiler.const_boolean(b),
//             Self::Number(n) => compiler.const_number(n),
//             Self::String(c) => compiler.const_string(&c),
//             Self::Ident(i) => compiler.const_symbol(&i),
//             Self::Application(a) => a.flatten(compiler),
//             Self::Label(_) => todo!(),
//             Self::FnParam(p) => compiler.const_symbol(&format!("'{p}'").into()),
//         }
//     }
// }

// impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<Ast1> {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
//         fn list_to_tree<'ctx>(
//             list: Vec<Ast1>,
//             compiler: &mut Compiler<'_, 'ctx>,
//             n: usize,
//         ) -> (StructValue<'ctx>, Vec<Ast1>) {
//             if n == 0 {
//                 (compiler.hempty(), list)
//             } else {
//                 let left_size = (n - 1) / 2;
//                 let (left_tree, mut non_left_tree) = list_to_tree(list, compiler, left_size);
//
//                 let this = non_left_tree.remove(0).flatten(compiler);
//
//                 let right_size = n - (left_size + 1);
//                 let (right_tree, remaining) = list_to_tree(non_left_tree, compiler, right_size);
//                 (compiler.const_cons(left_tree, this, right_tree), remaining)
//             }
//         }
//         let n = self.len();
//         let partial_tree = list_to_tree(self, compiler, n);
//
//         partial_tree.0
//     }
// }

// TODO: maybe: do this as part of expander's compile, this would mean we would still need
// functionss in the ast
// and evaluator would need to work with this
// and we would need a way to get back regular for going back into expander
// so functions would still take produce Ast but evaluator would reduce Ast1 -> Ast as evaling b/c
// pair and syntax would still be Ast and theoritcally and of the other constants (number,strings,
// ....) only special forms cannot be Ast
#[derive(Debug, Clone, PartialEq)]
// TODO: #%expression form, begin0
pub enum Ast1 {
    // Maybe all not speical forms should be just Be
    // coreesponds to data
    Basic(Ast),
    // Boolean(Boolean),
    // Number(f64),
    // String(RC<str>),
    // Symbol(Symbol),
    // Label(RC<str>),
    // Pair(Box<Pair>),           // this could really be regular ast
    // Syntax(Box<Syntax<Ast1>>), // this could really be regular ast
    // TheEmptyList,              // this could really be regular ast
    // should simlify to ident or the like ...
    // FnParam(usize),

    // special forms
    If(Box<Ast1>, Box<Ast1>, Box<Ast1>),
    DefineValues(Vec<RC<str>>, Box<Ast1>),
    LetValues(Vec<(Vec<RC<str>>, Ast1)>, Box<Ast1>),
    LetRecValues(Vec<(Vec<RC<str>>, Ast1)>, Box<Ast1>),
    Lambda(Param, Box<Ast1>),
    Application(Box<Ast1>, Vec<Ast1>),
    // TODO: begin0
    Expression(Box<Ast1>),
    Begin(Vec<Ast1>),
    Set(RC<str>, Box<Ast1>),
    Quote(Ast),
    Stop(Option<Box<Ast1>>),
    Skip,
    Loop(Box<Ast1>),
    Module(RC<str>, ModuleType),
    Link(Label, Vec<Label>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(RC<str>);

impl fmt::Display for Ast1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: maybe datum to syntax it
            Self::Basic(s) => write!(f, "{s}"),
            Self::Application(f0, a0) => {
                write!(f, "({f0} {})", a0.iter().map(ToString::to_string).join(" "))
            }

            Self::If(cond, cons, alt) => write!(f, "(if {cond} {cons} {alt})"),
            Self::DefineValues(v, val) => todo!(),
            // write!(f, "(define {v} {val})"),
            Self::Lambda(param, body) => write!(f, "(lambda ({param} {body})",),
            Self::Begin(b) => write!(f, "(begin {})", b.iter().map(ToString::to_string).join(" ")),
            Self::Set(v, val) => write!(f, "(set! {v} {val})"),
            Self::Quote(q) => write!(f, ";{q}"),
            Self::Loop(l) => write!(f, "(loop {l}"),
            Self::Stop(s) => write!(
                f,
                "(stop{})",
                s.as_ref().map_or_else(String::new, |s| format!(" {s}"))
            ),
            Self::Skip => write!(f, "skip"),
            Self::LetRecValues(_, _) => todo!(),
            Self::Expression(_) => todo!(),
            Self::LetValues(_, _) => todo!(),
            Self::Link(_, _) => todo!(),
            Self::Module(name, _type) => write!(f, "(module {name})"),
        }
    }
}
pub fn immutable_add_to_vec<T>(mut v: Vec<T>, x: T) -> Vec<T> {
    v.push(x);
    v
}
pub fn immutable_add_to_hashmap<K: Hash + Eq, V>(
    mut h: HashMap<K, V>,
    k: K,
    v: V,
) -> HashMap<K, V> {
    h.insert(k, v);
    h
}
mod impl_transformer {

    use matcher_proc_macro::match_syntax;

    use crate::ast::{self, ast1::Ast1, Ast};

    type Error = String;

    ///  transformations happen during this phase:
    /// 1: all special forms are typified
    /// 2: applications are turned from linked list into lists
    impl TryFrom<Ast> for Ast1 {
        type Error = Error;

        fn try_from(value: Ast) -> Result<Self, Self::Error> {
            match value {
                Ast::Boolean(_)
                | Ast::Number(_)
                | Ast::String(_)
                | Ast::Symbol(_)
                | Ast::Label(_)
                | Ast::Syntax(_)
                | Ast::TheEmptyList => Ok(Self::Basic(value)),
                Ast::Pair(pair) => convert_pair(*pair),
                Ast::Function(function) => Err(format!("cannot have function, found {function}")),
            }
        }
    }

    fn convert_pair(pair: ast::Pair) -> Result<Ast1, String> {
        if let Ast::Symbol(ref s) = pair.1 {
            match s.0.to_string().as_str() {
                "if" => {
                    let m = match_syntax!((test then r#else))(pair.1)?;
                    todo!()
                }
                "define-values" => {
                    let m = match_syntax!(
                        (
                            (id:sym ...)
                            rhs
                        )
                    )(pair.1)?;

                    todo!()
                }
                "set!" => {
                    let m = match_syntax!( ( id:sym value))(pair.1)?;
                    todo!()
                }
                "quote" => todo!(),
                "begin" => todo!(),
                "lambda" => {
                    let m = match_syntax!(
                        ( formals body)
                    )(pair.1)?;
                    todo!()
                }
                "let-values" | "letrec-values" => {
                    let m = match_syntax!(
                        (
                            (((id:sym ...) rhs) ...)
                            body

                        )
                    )(pair.1)?;
                    todo!()
                }
                "link" => todo!(),
                // "stop" => todo!(),
                // "skip" => todo!(),
                // "loop" => todo!(),
                // "module" => todo!(),
                "quote-syntax" => todo!(),
                _ => pair_to_application(pair),
            }
        } else {
            pair_to_application(pair)
        }
    }

    fn pair_to_application(pair: ast::Pair) -> Result<Ast1, String> {
        pair.1
            .map_to_list_checked(TryInto::try_into)
            .map_err(|e| e.unwrap_or("not a list".to_string()))
            .and_then(|args| {
                pair.0
                    .try_into()
                    .map(|f| Ast1::Application(Box::new(f), args))
            })
    }

    // fn extend_if_found(name: impl fmt::Display, env: State) -> State {
    //     if let Some(i) = SPECIAL_FORMS.iter().position(|&x| x == name.to_string()) {
    //         {
    //             let mut v = env;
    //             let x = SPECIAL_FORMS[i];
    //             v.0.push(x);
    //             v
    //         }
    //     } else {
    //         env
    //     }
    // }

    // fn convert_begin(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     exps.into_iter()
    //         .try_fold((vec![], env), |(exps, env), current| {
    //             Ast1::transform(current, env)
    //                 .map(|(current, env)| (immutable_add_to_vec(exps, current), env))
    //         })
    //         .map(|(app, env)| (Ast1::Begin(app), env))
    // }

    // fn convert_quoted(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     if exps.len() != 1 {
    //         return Err("quoted expression can only contain single expression".to_string());
    //     }
    //     fn quote(exp: Ast) -> Ast1 {
    //         match exp {
    //             Ast::Bool(t) => Ast1::Boolean(t),
    //             Ast::Number(t) => Ast1::Number(t),
    //             Ast::String(t) => Ast1::String(t),
    //             Ast::Ident(t) => Ast1::Symbol(t),
    //             Ast::Application(t) => Ast1::Application(t.into_iter().map(quote).collect()),
    //             Ast::Label(t) => Ast1::Label(t),
    //             Ast::FnParam(t) => Ast1::FnParam(t),
    //         }
    //     }
    //     Ok((Ast1::Quote(Box::new(quote(exps[0].clone()))), env))
    // }

    // TODO: sets should only work if there is already a variable defined
    // fn convert_set(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     // TODO: set should only be allowed to be able to set non special forms
    //     if exps.len() != 2 {
    //         return Err("the set! form must follow (set! [var] [value])".to_string());
    //     }
    //
    //     let Ast::Ident(var) = exps[0].clone() else {
    //         return Err("the set! [var] must be a symbol".to_string());
    //     };
    //     Ast1::transform(exps[1].clone(), env).map(|(exp, env)| {
    //         (
    //             Ast1::Set(var.clone(), Box::new(exp)),
    //             extend_if_found(var, env),
    //         )
    //     })
    // }
    // TODO: we cannot shaddow an already defined variable unless where in a child scope
    // scope means in lambda or cond, so by extension let
    // this may mean we need our env simulation to be hierchical
    // fn convert_define(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     if exps.len() < 2 {
    //         return Err("the define form must follow (define [var] [value]) or (define ([var] [argc] <vararg>) exp+ )".to_string());
    //     }
    //     match exps[0].clone() {
    //         Ast::Application(a) => {
    //             if a.len() < 2 || a.len() > 3 {
    //                 return Err(
    //                     "the define form signature must follow ([var] [argc] <vararg>)".to_string(),
    //                 );
    //             }
    //             let Ast::Ident(i) = a[0].clone() else {
    //                 return Err("the define form [var] must be a symbol".to_string());
    //             };
    //             let env = extend_if_found(i.clone(), env);
    //             convert_lambda(
    //                 vec![Ast::Application(a[1..].to_vec())]
    //                     .into_iter()
    //                     .chain(exps[1..].to_vec())
    //                     .collect(),
    //                 env,
    //             )
    //             .map(|(exp, env)| (Ast1::Define(i, Box::new(exp)), env))
    //         }
    //         Ast::Ident(i) => {
    //             if exps.len() != 2 {
    //                 return Err(
    //                     "the define form (define [var] [value]) must follow not have anything else"
    //                         .to_string(),
    //                 );
    //             }
    //             let env = extend_if_found(i.clone(), env);
    //             Ast1::transform(exps[1].clone(), env)
    //                 .map(|(exp, env)| (Ast1::Define(i, Box::new(exp)), env))
    //         }
    //         _ => Err(
    //             "the first part of a define must be [var] or ([var] [argc] <varags>)".to_string(),
    //         ),
    //     }
    // }
    // fn convert_lambda(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     if exps.len() < 2 {
    //         return Err(
    //             "the lambda form must follow (lambda ([argc] <vararg>) exp+ ) ".to_string(),
    //         );
    //     }
    //     let (argc, vararg) = if let Ast::Application(app) = &exps[0] {
    //         match app.as_slice() {
    //             [Ast::Number(n), Ast::Ident(s)] if ["+".into(), "*".into()].contains(s) => (
    //                 *n,
    //                 if s.to_string().as_str() == "*" {
    //                     Some(Varidiac::AtLeast0)
    //                 } else {
    //                     Some(Varidiac::AtLeast1)
    //                 },
    //             ),
    //
    //             [Ast::Number(n)] => (*n, None),
    //             _ => todo!("self function should return result so self can error"),
    //         }
    //     } else {
    //         return Err("paramters in lambda does not take form ([argc] <varargs>) ".to_string());
    //     };
    //     let (body, _) = convert_begin(exps[1..].to_vec(), env.clone())?;
    //     Ok((Ast1::Lambda(argc as usize, vararg, Box::new(body)), env))
    // }
    // fn convert_if(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     if exps.len() != 3 {
    //         return Err(
    //             "the if form must follow (if [condition] [consequent] [alternative])".to_string(),
    //         );
    //     }
    //     Ast1::transform(exps[0].clone(), env).and_then(|(cond, env)| {
    //         Ast1::transform(exps[1].clone(), env).and_then(|(cons, env)| {
    //             Ast1::transform(exps[2].clone(), env).map(|(alt, env)| {
    //                 (Ast1::If(Box::new(cond), Box::new(cons), Box::new(alt)), env)
    //             })
    //         })
    //     })
    // }

    // fn convert_application(app: Vec<Ast>) -> Result<Ast1, Error> {
    //     match app.first() {
    //         Some(Ast::Symbol(i))
    //             if !env.0.contains(&i.to_string().as_str())
    //                 && SPECIAL_FORMS.contains(&i.to_string().as_str()) =>
    //         {
    //             // TODO: have constraints on where some special forms can be used/rededinfed to help with the approximation of where some special forms are redefined when using lazyness
    //             let exps = app[1..].to_vec();
    //             match i.to_string().as_str() {
    //                 "lambda" => convert_lambda(exps, env),
    //                 "define" => convert_define(exps, env),
    //                 "set!" => convert_set(exps, env),
    //                 "begin" => convert_begin(exps, env),
    //                 "if" => convert_if(exps, env),
    //                 "quote" => convert_quoted(exps, env),
    //                 "stop" => convert_return(exps, env),
    //                 "loop" => convert_loop(exps, env),
    //                 // "module" => convert_module(exps, env),
    //                 _ => unreachable!(),
    //             }
    //         }
    //
    //         Some(fst) => {
    //             let fst = Ast1::transform(fst.clone(), env)?;
    //             let fst = (vec![fst.0], fst.1);
    //             app[1..]
    //                 .iter()
    //                 .cloned()
    //                 .try_fold(fst, |(app, env), current| {
    //                     Ast1::transform(current, env)
    //                         .map(|(current, env)| (immutable_add_to_vec(app, current), env))
    //                 })
    //                 .map(|(app, env)| (Ast1::Application(app), env))
    //         }
    //         None => Err("application must have at least one argument".to_string()),
    //     }
    // }

    // fn convert_module(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     // TODO: requires modification of state type to include a hashmap of moduletype, vec ast,
    //     // and usize, so no inline modules hash to same thing
    //     // then if its an inline module have moduletype be inline wityh usize, increment usize and
    //     // parse inline module put in hashmap and put module and name as module to return
    //     // if its file read and parse file put in hasmap as module type file with file path
    //     // then return module type and name as module in ast
    //     let mut iter = exps.into_iter();
    //     let name = iter.next().ok_or("module missing name".to_string())?;
    //     let Ast::Ident(name) = name else {
    //         return Err(format!("module name is not an ident {name}"));
    //     };
    //     match (iter.next(), iter.collect_vec()) {
    //         (Some(Ast::String(path)), v) if v.is_empty() => {
    //             let mut file = File::open(&*path)
    //                 .map_err(|e| format!("cannot open module {name} from path {path}: {e}"))?;
    //             let mut buf = String::new();
    //             file.read_to_string(&mut buf)
    //                 .map_err(|e| format!("cannot read module {name} from path {path}: {e}"))?;
    //             // TODO: combine ast1 -> to ast2 and macro expansion + link finding into one phase
    //             // so that extenral modules can share links with other code
    //             let (exps, map) = parse_and_expand(&buf)
    //                 .map_err(|e| format!("cannot parse module {name} from path {path}: {e:?}"))?;
    //             let (exps, (_, i, map)) = exps
    //                 .into_iter()
    //                 .transform::<Ast1>((vec![], env.1, env.2))
    //                 .transform_all::<Vec<_>>()
    //                 .map_err(|e| {
    //                     format!("cannot transform module {name} from path {path}: {e:?}")
    //                 })?;
    //             let module_type = ModuleType::Path(path);
    //             let map = immutable_add_to_hashmap(map, module_type.clone(), exps);
    //             let state = (env.0, i + 1, map);
    //             Ok((Ast1::Module(name, module_type), state))
    //         }
    //         (Some(expr), v) => {
    //             let exps = chain!([expr], v);
    //             let (exps, (_, i, map)) = exps
    //                 .into_iter()
    //                 .transform::<Ast1>((vec![], env.1, env.2))
    //                 .transform_all::<Vec<_>>()
    //                 .map_err(|e| format!("cannot transform module {name}: {e:?}"))?;
    //             let module_type = ModuleType::Inline(env.1);
    //             let map = immutable_add_to_hashmap(map, module_type.clone(), exps);
    //             let state = (env.0, i + 1, map);
    //             Ok((Ast1::Module(name, module_type), state))
    //         }
    //         (None, _) => {
    //             // TODO: empty inline modules
    //             let module_type = ModuleType::Inline(env.1);
    //             let map = immutable_add_to_hashmap(env.2, module_type.clone(), vec![]);
    //             let state = (env.0, env.1 + 1, map);
    //             Ok((Ast1::Module(name, module_type), state))
    //         }
    //     }
    // }

    // fn convert_loop(exps: Vec<Ast>, env: State) -> Result<(Ast1, State), Error> {
    //     match &exps[..] {
    //         [body] => Ast1::transform(body.clone(), env)
    //             .map(|(ast, env)| (Ast1::Loop(Box::new(ast)), env)),
    //         _ => Err(format!(
    //             "loop with to many statements {}",
    //             exps.into_iter()
    //                 .map(|e| e.to_string())
    //                 .collect::<Vec<_>>()
    //                 .join(" ")
    //         )),
    //     }
    // }

    // fn convert_return(exps: Vec<Ast>, state: State) -> Result<(Ast1, State), Error> {
    //     if exps.len() > 1 {
    //         Err(format!(
    //             "Stop's can only hace at most one expression found {} expressions: {}",
    //             exps.len(),
    //             exps.into_iter()
    //                 .map(|e| e.to_string())
    //                 .collect::<Vec<_>>()
    //                 .join(" ")
    //         ))
    //     } else {
    //         let (s, state) = match exps.first() {
    //             Some(s) => {
    //                 Ast1::transform(s.clone(), state).map(|(s, state)| (Some(Box::new(s)), state))
    //             }
    //             None => Ok((None, state)),
    //         }?;
    //         Ok((Ast1::Stop(s), state))
    //     }
    // }
}
