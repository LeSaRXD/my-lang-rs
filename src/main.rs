use std::io::{stdin, stdout, Write};

use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
	let mut input = String::new();

	let mut parser = Parser::new();
	loop {
		print!("> ");
		stdout().flush().unwrap();
		stdin().read_line(&mut input).unwrap();

		if input.trim() == "exit" {
			break;
		}
		if input.ends_with("\\\n") {
			continue;
		}

		input.pop();

		let ast = match parser.produce_ast(&input) {
			Ok(ast) => ast,
			Err(e) => panic!("Could not create AST: {e}"),
		};
		for expr in ast {
			println!("{expr}");
			println!("{expr:?}");
		}
		input.clear();
	}
}
