use std::fmt;

#[derive(Debug, Clone)]
pub struct OperatorParsingError;

impl fmt::Display for OperatorParsingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error parsing operator")
	}
}

type Result<T> = std::result::Result<T, OperatorParsingError>;

/**
 * Represents a finite set of possible operators
 */
pub enum Operator {
	Plus,
	Minus,
	Times,
	Divide
}

#[allow(dead_code)]
/**
 * Parse an operator from a character
 */
pub fn parse_operator(character: char) -> Result<Operator> {
	match character {
		'+' => Ok(Operator::Plus),
		'-' => Ok(Operator::Minus),
		'*' => Ok(Operator::Times),
		'/' => Ok(Operator::Divide),
		_ => Err(OperatorParsingError)
	}
}
