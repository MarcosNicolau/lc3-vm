use std::env;

use lc3_vm::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut vm = VM::new();
    vm.run(file_path);
}
