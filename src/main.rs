use std::env;
use std::fs;
use std::io::Write;

use rust_cc::syntax::Node;
use rust_cc::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let code = fs::read_to_string(filename).unwrap();
    let tokens = tokenizer::tokenize(&code);
    let program = parser::parse_program_tokens(tokens).unwrap();
    let asm_code = program.to_asm();
    println!("{}", asm_code);
    let mut file = fs::File::create("compiled.s").unwrap();
    file.write_all(asm_code.as_bytes()).unwrap();
}
