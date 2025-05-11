#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
// #![allow(clippy::similar_names, dead_code, unused)]

use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use ast::{scope::Scope, Ast, Symbol};
use expander::{expand_context::ExpandContext, Expander};
//use codegen::{
//    register_to_llvm::CodeGen,
//    sicp::{Linkage, Register},
//};
use inkwell::passes::PassManager;
use simple_file_logger::LogLevel;

use clap::{arg, Parser, Subcommand};

pub mod ast;
//mod codegen;

mod evaluator;
mod expander;
pub mod lexer;
//mod macros;
pub mod pc;
mod primitives;
mod reader;

#[cfg(feature = "multi-threaded")]
pub mod interior_mut {
    use std::sync::{Arc, Mutex};

    pub type RC<T> = Arc<T>;
    pub type MUTEX<T> = Mutex<T>;
}

#[cfg(not(feature = "multi-threaded"))]
pub mod interior_mut {
    use std::{cell::RefCell, rc::Rc};

    pub type RC<T> = Rc<T>;
    pub type MUTEX<T> = RefCell<T>;
}
#[derive(Parser, Debug)]
#[clap(author = "mendelsshop", version, about, long_about = None, name = "Everything")]
pub struct Args {
    #[clap(subcommand)]
    arg: ArgType,
    #[arg(long, default_value=None)]
    log_level: Option<LogLevel>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ArgType {
    /// Start an `Everything` repl
    Repl,
    /// Compile some code
    Compile {
        filename: String,
        /// Output file name excluding file extension
        output: String,
    },
    /// Run some code
    Run {
        filename: String,
    },
    Expand {
        filename: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    simple_file_logger::init_logger!("everything-lang", args.log_level.unwrap_or_default())?;
    match args.arg {
        ArgType::Repl => repl(),
        ArgType::Compile { filename, output } => compile(&filename, &output),
        ArgType::Run { filename } => run(&filename),
        ArgType::Expand { filename } => expand(&filename),
    };
    Ok(())
}

fn init_function_optimizer<'ctx>(
    module: &inkwell::module::Module<'ctx>,
) -> PassManager<inkwell::values::FunctionValue<'ctx>> {
    let fpm = PassManager::create(module);
    fpm.add_new_gvn_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_aggressive_inst_combiner_pass();
    // // doesn't work with current goto implementation (non sicp)
    fpm.add_cfg_simplification_pass();
    fpm.add_aggressive_dce_pass();
    fpm.add_instruction_simplify_pass();

    fpm.add_verifier_pass();
    fpm.add_bit_tracking_dce_pass();
    fpm.add_merged_load_store_motion_pass();
    fpm.add_ind_var_simplify_pass();
    // // doesn't work with current goto implementation (non sicp)
    fpm.add_jump_threading_pass();

    fpm.add_scalarizer_pass();
    fpm.add_tail_call_elimination_pass();

    fpm.initialize();
    fpm
}

const fn repl() {}

const fn run(file: &str) {}

fn expand(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    //let program = parse_and_expand(&contents).unwrap();
    //println!("{program:?}");
}

fn compile(file: &str, out: &str) {
    let mut expander = Expander::new();
    // let env = CompileTimeEnvoirnment::new();
    let contents = fs::read_to_string(file).unwrap();
    let ns = expander.namespace();
    let ctx = ExpandContext::new(ns.clone());
    for ele in lexer::everything_parse(&contents).unwrap() {
        let ele = expander.namespace_syntax_introduce(ele.datum_to_syntax(None, None, None));
        let ele = expander.expand(ele, ctx.clone()).unwrap();
        let ele = expander.compile(ele, &ns).unwrap();
        println!("{ele}");
    }
    //let program = parse_and_expand(&contents).unwrap();
    //let links = MultiMap::from(program.1.into_iter().map(|(k, ks)| (ks, k.clone(), k)));
    //let (ast, _): (Vec<_>, _) = program
    //    .0
    //    .into_iter()
    //    .transform::<Ast2>((vec![], 0, HashMap::new()))
    //    .transform_all()
    //    .unwrap();
    //let (ast, _): (Vec<_>, _) = ast
    //    .into_iter()
    //    .transform::<Ast3>(links)
    //    .transform_all()
    //    .unwrap();
    //let (ast, _): (Vec<_>, _) = ast
    //    .into_iter()
    //    .transform::<Ast4>(())
    //    .transform_all()
    //    .unwrap();
    //eprintln!("{}\n", ast.iter().map(|e| format!("{e:?}")).join("\n"));
    //let ir: Vec<_> = ast
    //    .into_iter()
    //    .flat_map(|expr| {
    //        codegen::sicp::compile(expr, Register::Val, Linkage::Next, Linkage::Next)
    //            .instructions()
    //            .to_vec()
    //    })
    //    .collect();
    //eprintln!("{}", ir.iter().map(ToString::to_string).join("\n"));
    //let context = Context::create();
    //let module = context.create_module(file);
    //let builder = context.create_builder();
    //let fpm = init_function_optimizer(&module);
    //let mut codegen = CodeGen::new(&context, &builder, &module, &fpm);
    //codegen.compile(ir);
    //println!("\n{}", codegen.export_ir());
}

// use ast::{scope::Scope, Ast, Symbol};
// use expander::{binding::CompileTimeEnvoirnment, Expander};

trace::init_depth_var!();

// use trace::trace;
// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// struct Scope(usize);
#[derive(Debug)]
struct UniqueNumberManager(usize);

impl UniqueNumberManager {
    fn next() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    fn new_scope() -> Scope {
        Scope(Self::next(), Rc::new(RefCell::new(HashMap::new())))
    }

    fn gen_sym(name: impl ToString) -> Symbol {
        Symbol((name.to_string() + &Self::next().to_string()).into())
    }
}

impl Ast {
    // pub fn list_with_length(self, length: usize) -> Result<Vec<Ast>, Ast> {
    //     match self {
    //         Self::List(l) if l.len() == length => Ok(l),
    //         _ => Err(self),
    //     }
    // }

    // pub fn datum_to_syntax(self) -> Self {
    //     match self {
    //         Self::List(l) => Self::List(l.into_iter().map(Self::datum_to_syntax).collect()),
    //         Self::Syntax(_) => self,
    //         Self::Symbol(s) => Self::Syntax(Syntax::new(s)),
    //         _ => self,
    //     }
    // }
    // fn syntax_to_datum(self) -> Self {
    //     match self {
    //         Self::List(l) => Self::List(l.into_iter().map(Self::syntax_to_datum).collect()),
    //         Self::Syntax(Syntax(s, _)) => Self::Symbol(s),
    //         _ => self,
    //     }
    // }
    // fn identifier(&self) -> bool {
    //         matches!(self, Self::Syntax(_))
    //     }
}

// fn main() {
//     let mut reader = reader::Reader::new();
//     let newline = || {
//         let mut stdin = BufReader::new(std::io::stdin());
//         let mut input = String::new();
//         // flush the screen
//         std::io::stdout().flush().unwrap();
//         stdin.read_line(&mut input).unwrap();
//         input
//     };
//     let mut expander = Expander::new();
//     loop {
//         print!("\n>> ",);

//         let ast = reader
//             .read_with_continue(newline)
//             .inspect(|e| println!("after reader: {e}"))
//             .and_then(|ast| {
//                 expander.expand(
//                     expander.namespace_syntax_introduce(ast.datum_to_syntax(None)),
//                     CompileTimeEnvoirnment::new(),
//                 )
//             })
//             .inspect(|e| println!("after expansion: {e}"))
//             .and_then(|ast| expander.compile(ast))
//             .inspect(|e| println!("after expansion part 2: {e}"))
//             .and_then(|ast| expander.run_time_eval(ast));
//         match ast {
//             Ok(expr) => println!("{expr}"),
//             Err(e) => println!("{e}"),
//         }
//     }
// }
