use std::fmt;

#[derive(Debug, Clone)]
pub struct NumberParsingError;

impl fmt::Display for NumberParsingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Unable to parse number")
	}
}

type Result<T> = std::result::Result<T, NumberParsingError>;

#[derive(Debug)]
pub enum Number {
	Int(isize),
	Float(f32)
}


pub fn parse_number(line: String, start_at: usize) -> (Result<Number>, usize) {
	let mut is_float = false;
	let mut str_value = String::new();

	for c in line.chars().skip(start_at) {
		match c {
			// Add numbers to read value
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => str_value.push(c),

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
			_ => match str_value.len() {
				// Handle if no chars were read
				0 => return (Err(NumberParsingError), 0),

				_ => {
					// Convert to float and return
					if is_float {
						return if let Ok(val) = str_value.parse::<f32>() {
							(Ok(Number::Float(val)), str_value.len())
						} else {
							(Err(NumberParsingError), str_value.len())
						}
					}

					// Convert to int and return
					return if let Ok(val) = str_value.parse::<isize>() {
						(Ok(Number::Int(val)), str_value.len())
					} else {
						(Err(NumberParsingError), str_value.len())
					}
				}
			}
		}
	}

	// Default, should not happen
	(Err(NumberParsingError), str_value.len())
}
