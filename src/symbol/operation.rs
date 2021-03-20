use std::fmt;
use super::parser::number::Number;
use super::parser::operator::Operator;

#[derive(Debug, Clone)]
pub struct OperationError;

impl fmt::Display for OperationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "An error occured while executing an operation")
	}
}

type Result<T> = std::result::Result<T, OperationError>;

/**
 * A value in an operation
 */
#[derive(PartialEq, Clone, Copy)]
pub enum Operand {
	Number(Number),
	Unit()
}

impl fmt::Display for Operand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Operand::Number(number) => write!(f, "{}", number),
			Operand::Unit() => write!(f, "()")
		}
	}
}

/**
 * An operation to execute
 */
pub struct Operation {
	pub left: Operand,
	pub right: Operand,
	pub operator: Operator
}

impl Operation {
	fn execute_addition(&self) -> Result<Operand> {
		match (self.left, self.right) {
			(Operand::Number(Number::Int(a)), Operand::Number(Number::Int(b))) => Ok(Operand::Number(Number::Int(a + b))),
			(Operand::Number(Number::Float(a)), Operand::Number(Number::Float(b))) => Ok(Operand::Number(Number::Float(a + b))),
			(Operand::Number(Number::Float(a)), Operand::Number(Number::Int(b))) => Ok(Operand::Number(Number::Float(a + b as f32))),
			(Operand::Number(Number::Int(a)), Operand::Number(Number::Float(b))) => Ok(Operand::Number(Number::Float(a as f32 + b))),
			_ => Err(OperationError)
		}
	}

	fn execute_multiplication(&self) -> Result<Operand> {
		match (self.left, self.right) {
			(Operand::Number(Number::Int(a)), Operand::Number(Number::Int(b))) => Ok(Operand::Number(Number::Int(a * b))),
			(Operand::Number(Number::Float(a)), Operand::Number(Number::Float(b))) => Ok(Operand::Number(Number::Float(a * b))),
			(Operand::Number(Number::Float(a)), Operand::Number(Number::Int(b))) => Ok(Operand::Number(Number::Float(a * (b as f32)))),
			(Operand::Number(Number::Int(a)), Operand::Number(Number::Float(b))) => Ok(Operand::Number(Number::Float((a as f32) * b))),
			_ => Err(OperationError)
		}
	}

	pub fn execute(&self) -> Result<Operand> {
		match self.operator {
			Operator::Plus => self.execute_addition(),
			Operator::Times => self.execute_multiplication(),
			_ => Err(OperationError),
		}
	}	
}
