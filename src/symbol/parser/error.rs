use std::fmt;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "parsing error")
	}
}
