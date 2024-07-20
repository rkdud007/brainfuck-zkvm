use std::{fs, io::stdin};

use machine::Machine;
pub mod instruction;
pub mod machine;
pub mod register;
pub mod crypto;

fn main() {
    println!("Which brainfuck file you want to execute?");
    let input = &mut String::new();
    let stdin = stdin();
    let _ = stdin.read_line(input).unwrap();
    let code = fs::read_to_string(format!("examples/{}.bf", input.trim()))
        .unwrap()
        .replace(' ', "");
    let trimmed_code = code.chars().filter(|c| !c.is_whitespace()).collect();
    let mut bf_vm = Machine::new(trimmed_code);
    bf_vm.execute();
}
