pub mod error;
pub mod value;

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use error::{RuntimeError, RuntimeOperation};
use value::RuntimeValue;

use crate::ast::{
	binary_expr::{BinaryExpression, BinaryOp},
	expression::Expression,
	unary_expr::{UnaryExpression, UnaryOp},
};

pub type RuntimeResult = Result<RuntimeValue, RuntimeError>;

pub struct Runtime;

impl Runtime {
	pub fn evaluate(expr: Expression) -> RuntimeResult {
		match expr {
			Expression::Program(program) => Self::evaluate_program(program),
			Expression::LiteralNumber(number) => Ok(RuntimeValue::Number(number)),
			Expression::LiteralString(string) => Ok(RuntimeValue::String(string.into_string())),
			Expression::Identifier(_) => todo!(),
			Expression::Unary(unary) => Self::evaluate_unary(unary),
			Expression::Binary(binary) => Self::evaluate_binary(binary),
			Expression::Unit => Ok(RuntimeValue::Unit),
		}
	}

	fn evaluate_program(program: Vec<Expression>) -> RuntimeResult {
		let mut last = RuntimeValue::Unit;
		for expr in program {
			last = Self::evaluate(expr)?;
		}
		Ok(last)
	}

	fn evaluate_unary(unary: UnaryExpression) -> RuntimeResult {
		use UnaryOp::*;

		let right = Self::evaluate(*unary.right)?;
		match unary.operator {
			Plus => right.pos(),
			Minus => right.neg(),
		}
	}

	fn evaluate_binary(binary: BinaryExpression) -> RuntimeResult {
		use BinaryOp::*;
		use RuntimeError::*;

		let left = Self::evaluate(*binary.left)?;
		let right = Self::evaluate(*binary.right)?;

		match binary.operator {
			Add => left.add(right),
			Subtract => left.sub(right),
			Multiply => left.mul(right),
			Divide => left.div(right),
			Modulo => left.rem(right),
			Equals => {
				use std::mem::discriminant;
				if discriminant(&left) != discriminant(&right) {
					Err(UnsupportedOperation(RuntimeOperation::Binary(
						left,
						BinaryOp::Equals,
						right,
					)))
				} else {
					Ok(RuntimeValue::Number(crate::numeric::Numeric::Int(
						left.eq(&right) as i128,
					)))
				}
			}
		}
	}
}
