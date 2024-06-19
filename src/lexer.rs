
use regex::Regex;

use super::{Token, CalcError, CalcErrorType};

pub fn tokenize(input_str: &str) -> Result<Vec<Token>, CalcError> {
	println!("Tokenizing");

	let ident_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_.]*$").expect("Failed to generate ident regex");
	let num_re = Regex::new(r"^[0-9]+(?:\.[0-9]+)?(?:e[0-9]+)?$").expect("Failed to generate number regex");

	let mut tokens: Vec<Token> = Vec::new();
	let mut remaining = input_str;

	loop {
		println!("\nIS: {remaining}");
		if remaining.len() <= 0 {
			break;
		}

		let tok: Token;
		let mut found_tok = false;
		let mut current_tok = remaining.to_string();

		while current_tok.len() > 0 {
			if ident_re.is_match(&current_tok) {
				tok = Token::Ident(current_tok.clone());
			} else if num_re.is_match(&current_tok) {
				tok = Token::Literal(current_tok.parse::<f64>().expect("Failed to parse number literal"));
			} else if current_tok.eq("(") {
				tok = Token::LParen;
			} else if current_tok.eq(")") {
				tok = Token::RParen;
			} else if current_tok.eq("+") {
				tok = Token::Plus;
			} else if current_tok.eq("-") {
				tok = Token::Minus;
			} else if current_tok.eq("*") {
				tok = Token::Multiply;
			} else if current_tok.eq("/") {
				tok = Token::Divide;
			} else if current_tok.eq("%") {
				tok = Token::Percent;
			} else if current_tok.eq("^") {
				tok = Token::Caret;
			} else if current_tok.eq("!") {
				tok = Token::Exclamation;
			} else if current_tok.eq(",") {
				tok = Token::Comma;
			} else if is_whitespace(&current_tok) {
				// white space, skipping
				found_tok = true;
				break;
			} else {
				current_tok.pop();
				continue;
			}
			tokens.push(tok);
			found_tok = true;
			break;
		}
		if !found_tok {
			// generating padding for fancy error message
			let pos = input_str.len() - remaining.len();
			let pad = std::iter::repeat(" ").take(pos).collect::<String>();

			return Err(CalcError {
				error_type: CalcErrorType::ParserError,
				msg: String::from(format!("Unexpected token\n| \n| {input_str}\n| {pad}└─ here")),
			})
		}
		println!("Before: {remaining}");
		remaining = &remaining[current_tok.len()..];
		println!("After: {remaining} ({})", remaining.len());
	}
	println!("Done");

	Ok(tokens)
}

fn is_whitespace(s: &String) -> bool {
	for c in s.chars() {
		if !c.is_whitespace() {
			return false;
		}
	}
	return true;
}