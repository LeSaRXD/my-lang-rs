pub mod error;
pub mod inner_value;
pub mod value;

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use error::{RuntimeError, RuntimeOperation};
use value::{Pos, RuntimeValue};

use crate::{
	ast::{
		binary_expr::{BinaryExpression, BinaryOp},
		expression::Expression,
		unary_expr::{UnaryExpression, UnaryOp},
	},
	environment::Env,
	numeric::Numeric,
};

pub type RuntimeResult = Result<RuntimeValue, RuntimeError>;

pub struct Runtime {
	global_env: Env,
}

impl Runtime {
	pub fn new(global_env: Env) -> Self {
		Self { global_env }
	}

	pub fn evaluate(&self, expr: Expression) -> RuntimeResult {
		use Expression::*;

		match expr {
			Program(program) => self.evaluate_program(program),
			LiteralNumber(number) => Ok(RuntimeValue::number(number)),
			LiteralString(string) => Ok(RuntimeValue::string(string.into_string())),
			Identifier(ident) => self.global_env.evaluate(&ident),
			Unary(unary) => self.evaluate_unary(unary),
			Binary(binary) => self.evaluate_binary(binary),
			Unit => Ok(RuntimeValue::unit()),
		}
	}

	fn evaluate_program(&self, program: Vec<Expression>) -> RuntimeResult {
		let mut last = RuntimeValue::unit();
		for expr in program {
			last = self.evaluate(expr)?;
		}
		Ok(last)
	}

	fn evaluate_unary(&self, unary: UnaryExpression) -> RuntimeResult {
		use UnaryOp::*;

		let right = self.evaluate(*unary.right)?;

		match unary.operator {
			Plus => right.pos(),
			Minus => right.neg(),
		}
	}

	fn evaluate_binary(&self, binary: BinaryExpression) -> RuntimeResult {
		use BinaryOp::*;
		use RuntimeError::*;

		let left = self.evaluate(*binary.left)?;
		let right = self.evaluate(*binary.right)?;

		match binary.operator {
			Add => left.add(right),
			Subtract => left.sub(right),
			Multiply => left.mul(right),
			Divide => left.div(right),
			Modulo => left.rem(right),
			Equals => {
				if left.same_type(&right) {
					Ok(RuntimeValue::number(Numeric::Int(left.eq(&right) as i128)))
				} else {
					Err(UnsupportedOperation(RuntimeOperation::Binary(
						left.inner().to_owned(),
						BinaryOp::Equals,
						right.inner().to_owned(),
					)))
				}
			}
		}
	}
}
