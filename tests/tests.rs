//! Integration tests

use scientific_calculator::{calculate, context::Context};

#[test]
fn test_simple_1() {
	let mut ctx = Context::new();
	assert_eq!(calculate("1+1", &mut ctx).unwrap(), 2.0);
}

#[test]
fn test_simple_2() {
	let mut ctx = Context::new();
	assert_eq!(calculate("5 * 10  / 25", &mut ctx).unwrap(), 2.0);
}

#[test]
fn test_simple_error() {
	let mut ctx = Context::new();
	assert!(calculate("5 + ", &mut ctx).is_err());
}

#[test]
fn test_scientific_notation() {
	let mut ctx = Context::new();
	assert_eq!(calculate("1.2345e10", &mut ctx).unwrap(), 12345000000.0);
}

#[test]
fn test_order_1() {
	let mut ctx = Context::new();
	assert_eq!(calculate("5 + 5 * 2", &mut ctx).unwrap(), 15.0);
}

#[test]
fn test_order_2() {
	let mut ctx = Context::new();
	assert_eq!(calculate("1 + 3 - 2 * 2^2 / (3! - 1)", &mut ctx).unwrap(), 2.4);
}

#[test]
fn test_paren_1() {
	let mut ctx = Context::new();
	assert_eq!(calculate("(5)", &mut ctx).unwrap(), 5.0);
}

#[test]
fn test_paren_2() {
	let mut ctx = Context::new();
	assert_eq!(calculate("-((5 + (-5)) * 2 - (4.5 / 9))", &mut ctx).unwrap(), 0.5);
}

#[test]
fn test_paren_3() {
	let mut ctx = Context::new();
	assert_eq!(calculate("((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((5))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))", &mut ctx).unwrap(), 5.0);
}

#[test]
fn test_paren_error() {
	let mut ctx = Context::new();
	assert!(calculate("(5", &mut ctx).is_err());
}

#[test]
fn test_var_assignment() {
	let mut ctx = Context::new();
	assert!(calculate("a = 5 + 5", &mut ctx).is_ok());
	assert_eq!(calculate("a", &mut ctx).unwrap(), 10.0);
}

#[test]
fn test_var_assignment_error() {
	let mut ctx = Context::new();
	assert!(calculate("pi = 3.14", &mut ctx).is_err());
}

#[test]
fn test_var_const() {
	let mut ctx = Context::new();
	assert_eq!(calculate("e", &mut ctx).unwrap(), std::f64::consts::E);
}

#[test]
fn test_function_1() {
	let mut ctx = Context::new();
	assert_eq!(calculate("sqrt(4)", &mut ctx).unwrap(), 2.0);
}

#[test]
fn test_function_2() {
	let mut ctx = Context::new();
	assert_eq!(calculate("root(243, 5)", &mut ctx).unwrap(), 3.0);
}

#[test]
fn test_function_3() {
	let mut ctx = Context::new();
	assert_eq!(calculate("mean(2, 3, 4, 5)", &mut ctx).unwrap(), 3.5);
}

#[test]
fn test_function_error() {
	let mut ctx = Context::new();
	assert!(calculate("sin(pi, 1)", &mut ctx).is_err());
}

#[test]
fn test_pythagorean() {
	let mut ctx = Context::new();
	let _ = calculate("a = 2", &mut ctx);
	let _ = calculate("b = 5", &mut ctx);
	assert_eq!(calculate("sqrt(a^2 + b^2)", &mut ctx).unwrap(), (29f64).powf(0.5));
}