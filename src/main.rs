
use std::io::stdin;
use ansi_term::Colour::Red;

use scientific_calculator::{calculate, context::Context};

fn main() {
	let mut raw_input: String;
	let mut ctx = Context::new();
	loop {
		raw_input = String::new();
		match stdin().read_line(&mut raw_input) {
			Ok(_) => {}
			Err(e) => {
				println!("IO error: {}", e);
			}
		}
		let input_string = raw_input.trim();

		if input_string.eq("exit") { break; } 

		match calculate(input_string, &mut ctx) {
			Ok(result) => println!(" = {}", result),
			Err(e) => println!("{}", Red.paint(e.to_string())),
		}
	}
}
