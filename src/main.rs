mod lexer;
mod parser;
mod ast;
mod interpreter;

use clap::{Arg, Command};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::fs;

fn main() {
    let matches = Command::new("ibcspsuedolang")
        .version("1.0")
        .author("Ray <ray@example.com>")
        .about("Interpreter for IBC pseudocode")
        .arg(Arg::new("file")
            .help("The input file with IBC pseudocode")
            .required(true)
            .index(1))
        .arg(Arg::new("print-ast")
            .help("Print the AST and exit")
            .short('p')
            .long("print-ast")
            .takes_value(false))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    if matches.is_present("print-ast") {
        let interpreter = Interpreter::new();
        interpreter.print_ast(&ast, 0);
    } else {
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&ast);
    }
}
