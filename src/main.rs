
use std::env;
use rustyline::{history::History, DefaultEditor};
use ansi_term::Color::Red;

use scientific_calculator::{calculate, context::Context};

fn main() {

	let mut ctx = Context::new();

	// calculating 
	if env::args().len() > 1 {
		let mut input = String::new();
		for arg in env::args().skip(1) {
			input.push_str(&arg);
			input.push(' ');
		}
		println!("{input}");
		match calculate(input.as_str(), &mut ctx) {
			Ok(result) => println!(" = {result}"),
			Err(e) => println!("{}", Red.paint(e.to_string())),
		}
		return;
	}

	// CLI IO
	let mut rl = DefaultEditor::new().unwrap();
	loop {
		let readline = rl.readline("");
		if let Err(_) = &readline {
			break;
		}
		let input = readline.unwrap();
		let input = input.as_str();
		if input.eq_ignore_ascii_case("exit") { break; }
		if input.eq("") { continue; }

		match calculate(input, &mut ctx) {
			Ok(result) => {
				println!(" = {result}");
				rl.history_mut().add(input).expect("Failed to append to history");
			}
			Err(e) => { 
				println!("{}", Red.paint(e.to_string()));
			}
		}
	}

	// bye bye
}
