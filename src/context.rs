use crate::{CalcError, CalcErrorType};


pub struct Context {
	pub var_table: Vec<VarTableEntry>,
	pub history: Vec<HistEntry>
}
impl Context {
	pub fn new() -> Self {
		Self {
			var_table: vec![
				VarTableEntry {
					name: "pi".to_string(),
					value: std::f64::consts::PI,
					constant: true,
				},
				VarTableEntry {
					name: "e".to_string(),
					value: std::f64::consts::E,
					constant: true,
				}
			],
			history: Vec::new(),
		}
	}
	pub fn lookup_var(&self, query: String) -> Result<f64, CalcError> {
		// hard coded 'ans' variable
		if query.eq("ans") {
			if self.history.is_empty() {
				return Err(CalcError {
					error_type: CalcErrorType::UndefinedVariable,
					msg: "Cannot use \'ans\' without a previous equation".to_string(),
				});
			}
			for entry in self.history.clone().into_iter().rev() {
				if let Some(answer) = entry.result {
					return Ok(answer);
				}
			}
			return Err(CalcError {
				error_type: CalcErrorType::UndefinedVariable,
				msg: "Cannot use \'ans\' without a previous valid solution".to_string(),
			})
		}
		for entry in &self.var_table {
			if entry.name.eq(&query) {
				return Ok(entry.value);
			}
		}
		return Err(CalcError {
			error_type: CalcErrorType::UndefinedVariable,
			msg: format!("Undefined variable \'{}\'", query).to_string()
		});
	}
	pub fn assign_var(&mut self, query: String, val: f64) -> Result<(), CalcError> {
		for entry in &mut self.var_table {
			if entry.name.eq(&query) {
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
			name: query,
			value: val,
			constant: false,
		});
		return Ok(());
	}
}

pub struct VarTableEntry {
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

#[derive(Clone)]
pub struct HistEntry {
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