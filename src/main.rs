use std::io::{stdin, stdout, Write};

use environment::Env;
use parser::Parser;
use runtime::Runtime;

mod environment;
mod expression;
mod helpers;
mod lexer;
mod numeric;
mod parser;
mod runtime;

fn main() {
	let mut parser = Parser::new();
	let runtime = Runtime::new(Env::global());

	let mut input = String::new();
	let mut print_debug = false;

	println!("My language repl v0.1.0");
	loop {
		if print_debug {
			debug(&parser, &runtime);
		}

		print!("> ");
		stdout().flush().unwrap();
		stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"exit" => break,
			"clear" => {
				print!("{esc}[2J{esc}[2J{esc}[1;1H", esc = 27 as char);
				input.clear();
				continue;
			}
			"debug" => {
				print_debug = !print_debug;
				input.clear();
				continue;
			}
			_ => (),
		}

		if input.ends_with("\\\n") {
			continue;
		}

		input.pop();

		match parser.produce_ast(&input, print_debug) {
			Ok(ast) => {
				let res = runtime.evaluate(ast);
				match res {
					Ok(ok) => println!("{ok}"),
					Err(err) => eprintln!("Runtime error: {err}"),
				}
			}
			Err(err) => eprintln!("Parse error: {err}"),
		}

		input.clear();
	}
}

fn debug(parser: &Parser, runtime: &Runtime) {
	println!("{}", parser);
	println!("{}", runtime);
}
