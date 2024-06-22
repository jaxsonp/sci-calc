
use rustyline::{history::History, DefaultEditor};
use ansi_term::Color::Red;

use scientific_calculator::{calculate, context::Context};

fn main() {

	let mut ctx = Context::new();

	// IO
	let mut rl = DefaultEditor::new().unwrap();
	loop {
		let readline = rl.readline("");
		if let Err(_) = &readline {
			break;
		}
		let input = readline.unwrap();
		let input = input.as_str();
		if input.eq_ignore_ascii_case("exit") { break; }

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
