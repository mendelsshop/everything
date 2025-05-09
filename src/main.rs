#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(static_mut_refs)]
#![deny(clippy::use_self, rust_2018_idioms, clippy::missing_panics_doc)]
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use ast::{scope::Scope, Ast, Symbol};
use expander::Expander;

trace::init_depth_var!();

mod ast;
mod evaluator;
mod expander;
mod primitives;
mod reader;

#[derive(Debug)]
struct UniqueNumberManager;

impl UniqueNumberManager {
    fn next() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    fn new_scope() -> Scope {
        Scope(
            UniqueNumberManager::next(),
            Rc::new(RefCell::new(HashMap::new())),
        )
    }

    fn gen_sym(name: impl ToString) -> Symbol {
        Symbol((name.to_string() + &UniqueNumberManager::next().to_string()).into())
    }
}

fn main() {
    let mut reader = reader::Reader::new();
    let newline = || {
        let mut stdin = BufReader::new(std::io::stdin());
        let mut input = String::new();
        // flush the screen
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        input
    };
    let mut expander = Expander::new();
    loop {
        print!("\n>> ",);

        let ast = reader
            .read_with_continue(newline)
            .inspect(|e| println!("after reader: {e}"))
            .and_then(|ast| {
                let ns = expander.namespace();
                expander.eval(ast, ns)
            });
        match ast {
            Ok(expr) => println!("{expr}"),
            Err(e) => println!("{e}"),
        }
    }
}
