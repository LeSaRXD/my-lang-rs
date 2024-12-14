use lexer::Lexer;

mod lexer;

fn main() {
	let src = r#"Hello world
a = 123
b = "literal\b"
45.2 * (3 / 2)
.1111111"#;

	let parsed = Lexer::tokenize(src).expect("Could not tokenize");
	for item in parsed {
		println!("{item:?}");
	}
}
