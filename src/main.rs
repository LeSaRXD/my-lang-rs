use std::io::{stdin, stdout, Write};

use environment::Env;
use parser::Parser;
use runtime::Runtime;

mod environment;
mod expression;
mod lexer;
mod numeric;
mod parser;
mod runtime;

fn main() {
	let mut input = String::new();

	let mut parser = Parser::new();
	let environment = Env::global();
	let runtime = Runtime::new(environment);

	println!("My language repl v0.1.0");
	loop {
		print!("> ");
		stdout().flush().unwrap();
		stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"exit" => break,
			"clear" => {
				print!("{esc}[2J{esc}[2J{esc}[1;1H", esc = 27 as char);
				continue;
			}
			_ => (),
		}

		if input.ends_with("\\\n") {
			continue;
		}

		input.pop();

		match parser.produce_ast(&input) {
			Ok(ast) => {
				let res = runtime.evaluate(ast);
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
