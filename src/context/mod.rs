
use crate::{CalcError, CalcErrorType};

mod builtins;

pub struct Context {
	var_table: Vec<VarTableEntry>,
	history: Vec<HistEntry>,
	function_table: Vec<Function>
}
impl Context {
	pub fn new() -> Self {
		Self {
			var_table: builtins::get_consts(),
			function_table: builtins::get_functions(),
			history: Vec::new(),
		}
	}
	/*fn assign_var(&mut self, name: &String, val: f64) -> Result<(), CalcError> {
		for entry in self.var_table.iter_mut() {
			if entry.name.eq(name) {
				if entry.constant {
					return Err(CalcError {
						error_type: CalcErrorType::AssignmentError,
						msg: format!("Can't assign to constant \"{name}\""),
					});
				}
				entry.value = val;
				return Ok(());
			}
		}
		return Err(CalcError {
			error_type: CalcErrorType::AssignmentError,
			msg: format!("Can't assign to constant \"{name}\""),
		});
	}*/
	pub fn lookup_var(&self, query: &String) -> Option<Result<f64, CalcError>> {
		// answer variable
		if query.eq("ans") {
			if self.history.is_empty() {
				return Some(Err(CalcError {
					error_type: CalcErrorType::ParserError,
					msg: "Cannot use \'ans\' without a previous equation".to_string(),
				}));
			}
			for entry in self.history.clone().into_iter().rev() {
				if let Some(answer) = entry.result {
					return Some(Ok(answer));
				}
			}
			return Some(Err(CalcError {
				error_type: CalcErrorType::ParserError,
				msg: "Cannot use \'ans\' without a previous valid solution".to_string(),
			}));
		}

		// looking up var in table
		for entry in &self.var_table {
			if entry.name.eq(query) {
				return Some(Ok(entry.value));
			}
		}
		return None;
	}

	pub fn try_function(&self, name: &String, args: Vec<f64>) -> Option<Result<f64, CalcError>> {
		for f in self.function_table.iter() {
			if !f.name.eq(name) { continue; }
			if f.num_args != 0 && f.num_args != args.len() {
				return Some(Err(CalcError {
					error_type: CalcErrorType::ArgumentError,
					msg: "Invalid number of arguments".to_string(),
				}));
			}
			return Some((f.closure)(args));
		}
		return None;
	}

	pub fn assign_var(&mut self, query: &String, val: f64) -> Result<(), CalcError> {
		for entry in &mut self.var_table {
			if entry.name.eq(query) {
				if entry.constant {
					return Err(CalcError {
						error_type: CalcErrorType::AssignmentError,
						msg: format!("Can't assign value to constant \'{}\'", entry.name).to_string()
					});
				}
				entry.value = val;
				return Ok(());
			}
		}
		self.var_table.push(VarTableEntry {
			name: query.clone(),
			value: val,
			constant: false,
		});
		return Ok(());
	}
}

struct VarTableEntry {
	pub name: String,
	pub value: f64,
	pub constant: bool,
}
impl VarTableEntry {
	pub fn new(name: String, value: f64, constant: bool) -> Self {
		Self {
			name,
			value,
			constant
		}
	}
}

struct Function {
	name: String,
	num_args: usize,
	closure: Box<dyn Fn(Vec<f64>) -> Result<f64, CalcError>>
}

#[derive(Clone)]
struct HistEntry {
	input: String,
	result: Option<f64>,
}
impl HistEntry {
	pub fn new(input: String, result: Option<f64>) -> Self {
		Self {
			input,
			result
		}
	}
}