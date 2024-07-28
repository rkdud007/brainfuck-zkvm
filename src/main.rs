use std::{
    fs,
    io::{stdin, stdout},
};

use compiler::Compiler;
use machine::Machine;

pub mod compiler;
pub mod crypto;
pub mod instruction;
pub mod machine;
pub mod registers;

fn main() {
    println!("0️⃣ Which brainfuck file you want to execute?");
    let input = &mut String::new();
    let stdin = stdin();
    stdin.read_line(input).expect("Failed to read line");
    let target_file = format!("examples/{}.bf", input.trim());
    let code = fs::read_to_string(&target_file)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", target_file))
        .replace(' ', "");
    println!("Selected program: {}", target_file);
    println!("\n======================== ");
    println!("1️⃣ Compiling...");
    let mut bf_compiler = Compiler::new(code);
    let ins = bf_compiler.compile();
    println!("🔥 Instructions:\n ");
    print!("[");
    for (index, ins) in ins.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("{}", ins);
    }
    println!("]");
    println!("\n======================== ");
    println!("2️⃣ Executing program...");
    let stdout = stdout();
    let mut bf_vm = Machine::new(ins, stdin, stdout);
    println!("input: ");
    bf_vm.execute().unwrap();
    println!("\n ");
    let traces = bf_vm.get_trace();
    println!("🔥 Full execution trace:\n ");
    for trace in traces {
        println!("{:?}", trace);
    }
}
