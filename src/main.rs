mod inst;
mod mem;
mod num;
mod proc;
mod vm;

use crate::vm::VM;
use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();

    if args.len() < 2 {
        println!("Please provide a binary file.");
        exit(-1);
    }

    let path = match args.skip(1).next() {
        Some(path) => path,
        None => {
            return Err(format!("Can't parse provided arguments.").into());
        }
    };

    let mut vm = VM::new();
    vm.execute(&path)
}
