# sci-calc

An easy to use, fully functional scientific calculator library with command line interface.

```
$ sci-calc
2 + 2
 = 4
a = ans * pi
 = 12.566370614359172
cos(a)
 = 1
exit
$
```

### Features

-   Variable assignment and recall
-   Comprehensive built-in functions and constants
-   Command line interface with line-editing and history recall
-   Elegant, verbose error handling

## Library

Add this crate to your project using cargo:

```sh
cargo add sci-calc
```

Below is a quick program implementing the `sci-calc` crate, see the [full documentation](https://docs.rs/sci-calc/latest/sci_calc/) for details.

```rust
use std::io::stdin;
use sci_calc::{calculate, context::Context};

fn main() {
	let mut ctx = Context::new();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		match calculate(input.as_str(), &mut ctx) {
			Ok(result) => println!(" = {result}"),
			Err(e) => println!("{e}"),
		}
	}
}
```

## Command Line Interface

To compile from source:

```sh
cargo build --release --bin sci-calc
```

This will compile the binary to `target/release/sci-calc`. Run the binary with an equation as an argument to evaluate that expression as a one-off, or run it with no arguments to use the calculator as a REPL. Use ctrl-C or the _exit_ keyword to exit the program.

```
$ sci-calc 5 + 5
 = 10
$ sci-calc
5 + 5
 = 10
ans * 2
 = 20
exit
$
```

## Content

### Basic Arithmetic

```
5 + 5 * 2^3
 = 45
1.23e5 * 4.56e-6
 = 0.56088
20!
 = 2432902008176640000
```

Implemented operators in reverse order of precedence:

-   Additive: `+`, `-`
-   Multiplicative: `*`, `/`, `%` (modulus), `//` (floored divide)
-   Exponentiation: `^`
-   Factorial: `!`

### Variable recall

```
a = 1 + 1
 = 2
b = 2 * 2
 = 4
a + b
 = 6
ans + 4
 = 10
```

### Builtins

#### Vars:

```
e
 = 2.718281828459045
2 * pi
 = 6.283185307179586
ans / 2
 = 3.141592653589793
```

#### Functions:

```
sqrt(2)
 = 1.4142135623730951
ln(e)
 = 1
2 * asin(1)
 = 3.141592653589793
stddev(1, 2, 3)
 = 0.816496580927726
```

All implemented functions:

-   Square root `sqrt(x)` and _n_-root `root(x, root)`
-   Factorial `fac(x)`, _same as `!` operator_
-   Mean `mean(x, y, ...)` and standard deviation `stddev(x, y, ...)`
-   Trig functions: `sin(x)`, `cos(x)`, and `tan(x)`
    -   Inverse variants: `asin(x)`, ...
    -   Hyperbolic variants: `sinh(x)`, ...
    -   Inverse hyperbolic variants: `asinh(x)`, ...
-   Mininum `min(x, y)` and maximum `max(x, y)`
-   Natural log `ln(x)`, log base-10 `log10(x)`, and log base-_n_ `log(x, base)`
-   Absolute value `abs()`, round `round()`, floor `floor()`, and ceiling `ceil()`

### Errors

#### Parsing error examples

```
5 +
Parser error: Unexpected EOI

5 + + 5
Parser error: Unexpected token
| 5 + + 5
|     └── here

5@ + 5
Parser error: Invalid token
| 5@ + 5
|  └── here
```

#### Variable error examples

```
ans * 2
Calculation error: Cannot use 'ans' without a previous evaluated equation

pi = 3
Assignment error: Can't assign value to constant 'pi'

a + 5
Undefined identifier: Unknown variable 'a'

```
