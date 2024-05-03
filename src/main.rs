#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
// #![allow(clippy::similar_names, dead_code, unused)]

use std::{fs, vec};

use codegen::{
    register_to_llvm::CodeGen,
    sicp::{Linkage, Register},
};
use inkwell::{context::Context, passes::PassManager};
use itertools::Itertools;

use crate::{
    ast::{ast2::Ast2, ast3::Ast3, ast4::Ast4, IteratorTransformer},
    codegen::multimap::MultiMap,
    macros::parse_and_expand,
};
use clap::{Parser, Subcommand};

pub mod ast;
mod codegen;

pub mod lexer;
mod macros;
pub mod pc;
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

fn main() {
    let args = Args::parse();
    match args.arg {
        ArgType::Repl => repl(),
        ArgType::Compile { filename, output } => compile(&filename, &output),
        ArgType::Run { filename } => run(&filename),
        ArgType::Expand { filename } => expand(&filename),
    }
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

fn repl() {}

fn run(file: &str) {}

fn expand(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let program = parse_and_expand(&contents).unwrap();
    println!("{program:?}");
}

fn compile(file: &str, out: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let program = parse_and_expand(&contents).unwrap();
    let links = MultiMap::from(program.1.into_iter().map(|(k, ks)| (ks, k.clone(), k)));
    let (ast, _): (Vec<_>, _) = program
        .0
        .into_iter()
        .transform::<Ast2>(vec![])
        .transform_all()
        .unwrap();
    let (ast, _): (Vec<_>, _) = ast
        .into_iter()
        .transform::<Ast3>(links)
        .transform_all()
        .unwrap();
    let (ast, _): (Vec<_>, _) = ast
        .into_iter()
        .transform::<Ast4>(())
        .transform_all()
        .unwrap();
    eprintln!("{}\n", ast.iter().map(|e| format!("{e:?}")).join("\n"));
    let ir: Vec<_> = ast
        .into_iter()
        .flat_map(|expr| {
            codegen::sicp::compile(expr, Register::Val, Linkage::Next)
                .instructions()
                .to_vec()
        })
        .collect();
    eprintln!("{}", ir.iter().map(ToString::to_string).join("\n"));
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    let fpm = init_function_optimizer(&module);
    let mut codegen = CodeGen::new(&context, &builder, &module, &fpm);
    codegen.compile(ir);
    println!("\n{}", codegen.export_ir());
}
