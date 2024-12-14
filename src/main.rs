use std::io::{stdin, stdout, Write};

use ast::expression::Expression;
use parser::Parser;
use runtime::Runtime;

mod ast;
mod lexer;
mod numeric;
mod parser;
mod runtime;

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

		match parser.produce_ast(&input) {
			Ok(ast) => {
				let res = Runtime::evaluate(Expression::Program(ast));
				match res {
					Ok(ok) => println!("{ok}"),
					Err(err) => eprintln!("Error: {err}"),
				}
			}
			Err(err) => eprintln!("Error: {err}"),
		}

		input.clear();
	}
}
