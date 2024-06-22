
use libm::tgamma;

use super::{VarTableEntry, Function};

/// Constructor function for built in constants
pub fn get_consts() -> Vec<VarTableEntry> {
	vec![
		VarTableEntry {
			name: "pi".to_string(),
			value: std::f64::consts::PI,
			constant: true,
		},
		VarTableEntry {
			name: "e".to_string(),
			value: std::f64::consts::E,
			constant: true,
		},
		VarTableEntry {
			name: "inf".to_string(),
			value: std::f64::INFINITY,
			constant: true,
		},
		VarTableEntry {
			name: "nan".to_string(),
			value: std::f64::NAN,
			constant: true,
		}
	]
}

/// Constructor function for builtin functions
pub fn get_functions() -> Vec<Function> {
	vec![
		Function {
			name: String::from("sqrt"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::sqrt(args[0])) })
		},
		Function {
			name: String::from("root"),
			num_args: 2,
			closure: Box::new(|args| { Ok(f64::powf(args[0], 1.0 / args[1])) })
		},
		Function {
			name: String::from("fac"),
			num_args: 1,
			closure: Box::new(|args| { Ok(tgamma(args[0] + 1.0)) })
		},
		Function {
			name: String::from("mean"),
			num_args: 0,
			closure: Box::new(|args| { 
				let mut sum = 0.0;
				for arg in args.iter() {
					sum += arg;
				}
				Ok(sum / (args.len() as f64))
			})
		},
		Function {
			name: String::from("stddev"),
			num_args: 0,
			closure: Box::new(|args| { 
				let n = args.len() as f64;
				let mut sum = 0.0;
				for arg in args.iter() {
					sum += arg;
				}
				let mean = sum.div_euclid(n);
				let mut dividend = 0.0;
				for arg in args.iter() {
					dividend += (arg - mean).powf(2.0);
				}
				Ok(f64::sqrt(dividend / n))
			})
		},
		Function {
			name: String::from("min"),
			num_args: 2,
			closure: Box::new(|args| { Ok(f64::min(args[0], args[1])) })
		},
		Function {
			name: String::from("max"),
			num_args: 2,
			closure: Box::new(|args| { Ok(f64::max(args[0], args[1])) })
		},
		Function {
			name: String::from("abs"),
			num_args: 1,
			closure: Box::new(|args| { Ok(args[0].abs()) })
		},
		Function {
			name: String::from("round"),
			num_args: 1,
			closure: Box::new(|args| { Ok(args[0].round()) })
		},
		Function {
			name: String::from("floor"),
			num_args: 1,
			closure: Box::new(|args| { Ok(args[0].floor()) })
		},
		Function {
			name: String::from("ceil"),
			num_args: 1,
			closure: Box::new(|args| { Ok(args[0].ceil()) })
		},
		Function {
			name: String::from("ln"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::log(args[0], std::f64::consts::E)) })
		},
		Function {
			name: String::from("log10"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::log(args[0], 10.0)) })
		},
		Function {
			name: String::from("log"),
			num_args: 2,
			closure: Box::new(|args| { Ok(f64::log(args[0], args[1])) })
		},
		Function {
			name: String::from("sin"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::sin(args[0])) })
		},
		Function {
			name: String::from("cos"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::cos(args[0])) })
		},
		Function {
			name: String::from("tan"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::tan(args[0])) })
		},
		Function {
			name: String::from("sinh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::sinh(args[0])) })
		},
		Function {
			name: String::from("cosh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::cosh(args[0])) })
		},
		Function {
			name: String::from("tanh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::tanh(args[0])) })
		},
		Function {
			name: String::from("asin"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::asin(args[0])) })
		},
		Function {
			name: String::from("acos"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::acos(args[0])) })
		},
		Function {
			name: String::from("atan"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::atan(args[0])) })
		},
		Function {
			name: String::from("asinh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::asinh(args[0])) })
		},
		Function {
			name: String::from("acosh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::acosh(args[0])) })
		},
		Function {
			name: String::from("atanh"),
			num_args: 1,
			closure: Box::new(|args| { Ok(f64::atanh(args[0])) })
		},
	]
}