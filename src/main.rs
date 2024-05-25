use std::{
    fs,
    io::{stdin, stdout},
};

use compiler::Compiler;
use machine::Machine;

pub mod compiler;
pub mod instruction;
pub mod machine;

fn main() {
    println!("Which brainfuck file you want to execute?");
    let input = &mut String::new();
    let stdin = stdin();
    let stdout = stdout();
    let _ = stdin.read_line(input).unwrap();
    let code = fs::read_to_string(format!("examples/{}.bf", input.trim()))
        .unwrap()
        .replace(' ', "");
    let mut bf_compiler = Compiler::new(code);
    let ins = bf_compiler.compile();
    println!("{:#?}", ins);
    let mut bf_vm = Machine::new(ins, stdin, stdout);
    bf_vm.execute();
}
