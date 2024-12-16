use std::{
	fmt::{Debug, Display},
	mem::Discriminant,
	ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::{
	expression::{binary::BinaryOp, unary::UnaryOp},
	numeric::Numeric,
	runtime::error::{RuntimeError, RuntimeOperation},
};

use super::variable::Pos;

type InnerRuntimeResult = Result<RuntimeValue, RuntimeError>;

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
			Number(number) => Display::fmt(number, f),
			String(st) => Debug::fmt(st, f),
		}
	}
}

impl RuntimeValue {
	pub fn discriminant(&self) -> Discriminant<Self> {
		std::mem::discriminant(self)
	}
}

impl Pos for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn pos(self) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Unary;
		use RuntimeValue::*;

		match self {
			Unit => Err(UnsupportedOperation(Unary(UnaryOp::Plus, Unit))),
			number @ Number(_) => Ok(number.to_owned()),
			string @ String(_) => Err(UnsupportedOperation(Unary(
				UnaryOp::Plus,
				string.to_owned(),
			))),
		}
	}
}

impl Neg for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn neg(self) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Unary;
		use RuntimeValue::*;

		match self {
			Unit => Err(UnsupportedOperation(Unary(UnaryOp::Plus, Unit))),
			Number(number) => Ok(Number(number.to_owned().neg())),
			string @ String(_) => Err(UnsupportedOperation(Unary(
				UnaryOp::Plus,
				string.to_owned(),
			))),
		}
	}
}

impl Add<&RuntimeValue> for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn add(self, rhs: &RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l.to_owned().add(r.to_owned()))),
			(number @ Number(_), other) => Err(UnsupportedOperation(Binary(
				number.to_owned(),
				BinaryOp::Add,
				other.to_owned(),
			))),
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Add,
				other.to_owned(),
			))),
			(String(l), String(r)) => Ok(String(format!("{l}{r}"))),
			(string @ String(_), other) => Err(UnsupportedOperation(Binary(
				string.to_owned(),
				BinaryOp::Add,
				other.to_owned(),
			))),
		}
	}
}

impl Sub<&RuntimeValue> for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn sub(self, rhs: &RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l.to_owned().sub(r.to_owned()))),
			(number @ Number(_), other) => Err(UnsupportedOperation(Binary(
				number.to_owned(),
				BinaryOp::Subtract,
				other.to_owned(),
			))),
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Subtract,
				other.to_owned(),
			))),
			(string @ String(_), other) => Err(UnsupportedOperation(Binary(
				string.to_owned(),
				BinaryOp::Subtract,
				other.to_owned(),
			))),
		}
	}
}

impl Mul<&RuntimeValue> for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn mul(self, rhs: &RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l.to_owned().mul(r.to_owned()))),
			(number @ Number(_), other) => Err(UnsupportedOperation(Binary(
				number.to_owned(),
				BinaryOp::Multiply,
				other.to_owned(),
			))),
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Multiply,
				other.to_owned(),
			))),
			(string @ String(st), other @ &Number(Numeric::Int(i))) => {
				if i < 0 || i > usize::MAX as i128 {
					Err(UnsupportedOperation(Binary(
						string.to_owned(),
						BinaryOp::Multiply,
						other.to_owned(),
					)))
				} else {
					Ok(String(st.repeat(i as usize)))
				}
			}
			(string @ String(_), other) => Err(UnsupportedOperation(Binary(
				string.to_owned(),
				BinaryOp::Multiply,
				other.to_owned(),
			))),
		}
	}
}

impl Div<&RuntimeValue> for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn div(self, rhs: &RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l.to_owned().div(r.to_owned()))),
			(number @ Number(_), other) => Err(UnsupportedOperation(Binary(
				number.to_owned(),
				BinaryOp::Divide,
				other.to_owned(),
			))),
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Divide,
				other.to_owned(),
			))),
			(string @ String(_), other) => Err(UnsupportedOperation(Binary(
				string.to_owned(),
				BinaryOp::Divide,
				other.to_owned(),
			))),
		}
	}
}

impl Rem<&RuntimeValue> for &RuntimeValue {
	type Output = InnerRuntimeResult;
	fn rem(self, rhs: &RuntimeValue) -> Self::Output {
		use RuntimeError::*;
		use RuntimeOperation::Binary;
		use RuntimeValue::*;

		match (self, rhs) {
			(Number(l), Number(r)) => Ok(Number(l.to_owned().rem(r.to_owned()))),
			(number @ Number(_), other) => Err(UnsupportedOperation(Binary(
				number.to_owned(),
				BinaryOp::Modulo,
				other.to_owned(),
			))),
			(Unit, other) => Err(UnsupportedOperation(Binary(
				Unit,
				BinaryOp::Modulo,
				other.to_owned(),
			))),
			(string @ String(_), other) => Err(UnsupportedOperation(Binary(
				string.to_owned(),
				BinaryOp::Modulo,
				other.to_owned(),
			))),
		}
	}
}
