use std::{
	fmt::{Debug, Display},
	ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::{
	ast::{binary_expr::BinaryOp, unary_expr::UnaryOp},
	numeric::Numeric,
	runtime::error::{RuntimeError, RuntimeOperation},
};

use super::RuntimeResult;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
	Unit,
	Number(Numeric),
	String(String),
}

impl Display for RuntimeValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RuntimeValue::*;

		match self {
			Unit => f.write_str("_"),
			Number(num) => Display::fmt(num, f),
			String(st) => Debug::fmt(st, f),
		}
	}
}

impl RuntimeValue {
	pub fn pos(self) -> RuntimeResult {
		use RuntimeError::*;
		use RuntimeOperation::Unary;
		use RuntimeValue::*;

		match self {
			u @ Unit => Err(UnsupportedOperation(Unary(UnaryOp::Plus, u))),
			num @ Number(_) => Ok(num),
			st @ String(_) => Err(UnsupportedOperation(Unary(UnaryOp::Plus, st))),
		}
	}
}

impl Neg for RuntimeValue {
	type Output = RuntimeResult;
	fn neg(self) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Unary;
		use RuntimeValue::*;

		match self {
			u @ Unit => Err(UnsupportedOperation(Unary(UnaryOp::Plus, u))),
			Number(num) => Ok(Number(-num)),
			st @ String(_) => Err(UnsupportedOperation(Unary(UnaryOp::Plus, st))),
		}
	}
}

impl Add<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn add(self, rhs: RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l + r)),
			(num @ Number(_), other) => {
				Err(UnsupportedOperation(Binary(num, BinaryOp::Add, other)))
			}
			(Unit, other) => Err(UnsupportedOperation(Binary(Unit, BinaryOp::Add, other))),
			(String(mut l), String(r)) => {
				l.push_str(&r);
				Ok(String(l))
			}
			(st @ String(_), other) => Err(UnsupportedOperation(Binary(st, BinaryOp::Add, other))),
		}
	}
}

impl Sub<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn sub(self, rhs: RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l - r)),
			(num @ Number(_), other) => {
				Err(UnsupportedOperation(Binary(num, BinaryOp::Subtract, other)))
			}
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Subtract,
				other,
			))),
			(st @ String(_), other) => {
				Err(UnsupportedOperation(Binary(st, BinaryOp::Subtract, other)))
			}
		}
	}
}

impl Mul<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn mul(self, rhs: RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l * r)),
			(num @ Number(_), other) => {
				Err(UnsupportedOperation(Binary(num, BinaryOp::Multiply, other)))
			}
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Multiply,
				other,
			))),
			(String(s), other @ Number(Numeric::Int(i))) => {
				if i < 0 || i > usize::MAX as i128 {
					Err(UnsupportedOperation(Binary(
						String(s),
						BinaryOp::Multiply,
						other,
					)))
				} else {
					Ok(String(s.repeat(i as usize)))
				}
			}
			(st @ String(_), other) => {
				Err(UnsupportedOperation(Binary(st, BinaryOp::Multiply, other)))
			}
		}
	}
}

impl Div<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn div(self, rhs: RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l / r)),
			(num @ Number(_), other) => {
				Err(UnsupportedOperation(Binary(num, BinaryOp::Divide, other)))
			}
			(Unit, other) => Err(UnsupportedOperation(Binary(Unit, BinaryOp::Divide, other))),
			(st @ String(_), other) => {
				Err(UnsupportedOperation(Binary(st, BinaryOp::Divide, other)))
			}
		}
	}
}

impl Rem<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn rem(self, rhs: RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l % r)),
			(num @ Number(_), other) => {
				Err(UnsupportedOperation(Binary(num, BinaryOp::Modulo, other)))
			}
			(Unit, other) => Err(UnsupportedOperation(Binary(Unit, BinaryOp::Modulo, other))),
			(st @ String(_), other) => {
				Err(UnsupportedOperation(Binary(st, BinaryOp::Modulo, other)))
			}
		}
	}
}
