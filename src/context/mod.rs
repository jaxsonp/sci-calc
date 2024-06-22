
use crate::{CalcError, CalcErrorType};

mod builtins;

#[cfg(test)]
mod tests;

/// Object passed to the calculate() function to track state of the calculator, 
/// including defined variables, functions, and equation history
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

	/// Takes in a query string and returns an Option that is none if the variable
	/// doesn't exist in the var table. The option contains a result that will be
	/// Ok with the var's value if the var can be read from, otherwise an error.
	pub fn lookup_var(&self, query: &String) -> Option<Result<f64, CalcError>> {
		// answer variable
		if query.eq("ans") {
			if self.history.is_empty() {
				return Some(Err(CalcError {
					error_type: CalcErrorType::CalculationError,
					msg: "Cannot use \'ans\' without a previous equation".to_string(),
				}));
			}
			for entry in self.history.clone().into_iter().rev() {
				if let Some(answer) = entry.result {
					return Some(Ok(answer));
				}
			}
			return Some(Err(CalcError {
				error_type: CalcErrorType::CalculationError,
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

	/// This function looks up a function with the specified name, returning a None
	/// Option if the function doesn't exist, an Err inside the Option if there's
	/// an issue with the arguments, otherwise it executes the function with the given
	/// arguments and returns the answer
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

	/// This function triest to assign a value to variable, returning an empty Ok
	/// if successful, otherwise an Err
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

/// Represents a variable, whether builtin constant or user-defined
struct VarTableEntry {
	pub name: String,
	pub value: f64,
	pub constant: bool,
}

/// Represents a builtin function, contains the name, number of args, and a closure
/// that performs the function's operation
struct Function {
	name: String,
	num_args: usize,
	closure: Box<dyn Fn(Vec<f64>) -> Result<f64, CalcError>>
}

/// Represents a previously evaluated equation, used for recalling and for the
/// `ans` variable
#[derive(Clone)]
struct HistEntry {
	input: String,
	result: Option<f64>,
}
