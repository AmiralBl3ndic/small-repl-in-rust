mod symbol;

use std::io::{self};
use symbol::parser::number::{Number, parse_number};
use symbol::operation::{Operation, Operand};
use symbol::parser::operator::Operator;

fn main() -> Result<(), io::Error> {
    if let (Ok(val), _) = parse_number(String::from("az123.4"), 2) {
        println!("{}\n{}", Operand::Number(val), Operand::Unit());
    } else {
        println!("Unable to parse");
    }

    let op1 = Operation {
        left: Operand::Number(Number::Int(30)),
        right: Operand::Number(Number::Float(12.5f32)),
        operator: Operator::Plus
    };

    if let Ok(result) = op1.execute() {
        println!("{}", result);
    } else {
        println!("Cannot perform operation");
    }

    Ok(())
}
