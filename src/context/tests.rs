use super::*;

#[test]
fn test_create_context() {
	
	let _ctx = Context::new();
}

#[test]
fn test_lookup_var() {
	let ctx = Context::new();
	let res = ctx.lookup_var(&String::from("pi"));
	assert_eq!(res.unwrap().unwrap(), std::f64::consts::PI);
}

#[test]
fn test_lookup_var_failure() {
	let ctx = Context::new();
	let res = ctx.lookup_var(&String::from("smth"));
	assert!(res.is_none());
}

/*#[test]
fn test_lookup_var_ans() {
	let ctx = Context::new();
	let res = ctx.lookup_var(&String::from("ans"));
	assert!(false);
}*/

#[test]
fn test_lookup_var_ans_failure() {
	let ctx = Context::new();
	let res = ctx.lookup_var(&String::from("ans"));
	assert!(matches!(res.unwrap().unwrap_err().error_type, CalcErrorType::CalculationError));
}

#[test]
fn test_try_function() {
	let ctx = Context::new();
	let res = ctx.try_function(&String::from("sqrt"), vec![4.0]);
	assert_eq!(res.unwrap().unwrap(), 2.0);
}

#[test]
fn test_try_function_invalid_args() {
	let ctx = Context::new();
	let res = ctx.try_function(&String::from("sqrt"), vec![4.0, 5.0]);
	assert!(matches!(res.unwrap().unwrap_err().error_type, CalcErrorType::ArgumentError));
}

#[test]
fn test_try_function_unknown_function() {
	let ctx = Context::new();
	let res = ctx.try_function(&String::from("bad_function"), vec![]);
	assert!(res.is_none());
}

#[test]
fn test_assign_var() {
	let mut ctx = Context::new();
	let name = String::from("a");
	assert!(ctx.assign_var(&name, 5.0).is_ok());
	let res = ctx.lookup_var(&name);
	assert_eq!(res.unwrap().unwrap(), 5.0)
}

#[test]
fn test_assign_var_const() {
	let mut ctx = Context::new();
	let res = ctx.assign_var(&String::from("pi"), 5.0);
	assert!(matches!(res.unwrap_err().error_type, CalcErrorType::AssignmentError));
}