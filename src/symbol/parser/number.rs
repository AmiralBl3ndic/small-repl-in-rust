use std::fmt;

#[derive(Debug, Clone)]
pub struct NumberParsingError;

impl fmt::Display for NumberParsingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Unable to parse number")
	}
}

type Result<T> = std::result::Result<T, NumberParsingError>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
	Int(isize),
	Float(f32)
}

impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Number::Int(val) => write!(f, "Int: {}", val),
			Number::Float(val) => write!(f, "Float: {}", val)
		}
	}
}


pub fn parse_number(line: String, start_at: usize) -> (Result<Number>, usize) {
	let mut is_float = false;
	let mut str_value = String::new();

	let parse_and_return = | str_value: String, is_float: bool | -> (Result<Number>, usize) {
		if is_float {
			if let Ok(val) = str_value.parse::<f32>() {
				return (Ok(Number::Float(val)), str_value.len())
			}
		}

		// Convert to int and return
		if let Ok(val) = str_value.parse::<isize>() {
			return (Ok(Number::Int(val)), str_value.len())
		}

		(Err(NumberParsingError), str_value.len())
	};

	for c in line.chars().skip(start_at) {
		match c {
			// Add numbers to read value
			x if x.is_digit(10) => str_value.push(c),

			// Handle floating point values
			'.' => {
				// If a floating point has already been described, return an error
				if is_float {
					return (Err(NumberParsingError), str_value.len())
				}

				is_float = true;
				str_value.push(c);
			},

			// Handling non-number character, means end of parsing
			_ => {
				// Convert to float and return
				return parse_and_return(str_value, is_float);
			}
		}
	}

	// Should only reach here if for loop breaks
	return parse_and_return(str_value, is_float);
}
