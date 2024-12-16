use std::fmt::Display;

use crate::expression::{binary::BinaryOp, unary::UnaryOp};

use super::value::RuntimeValue;

#[derive(Debug, Clone)]
pub enum RuntimeOperation {
	Unary(UnaryOp, RuntimeValue),
	Binary(RuntimeValue, BinaryOp, RuntimeValue),
}

impl Display for RuntimeOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RuntimeOperation::*;

		match self {
			Unary(op, right) => write!(f, "{op}{right}"),
			Binary(left, op, right) => write!(f, "{left} {op} {right}"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
	UnsupportedOperation(RuntimeOperation),
	VariableNotDeclared(Box<str>),
	VariableTypeDoesntMatch(Box<str>),
	CannotMutateVariable(Box<str>),
}

impl Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RuntimeError::*;

		match self {
			UnsupportedOperation(op) => write!(f, "Unsupported operation: {op}"),
			VariableNotDeclared(ident) => write!(f, "Variable '{ident}' is not declared"),
			VariableTypeDoesntMatch(ident) => {
				write!(f, "Variable '{ident}' is of a different type")
			}
			CannotMutateVariable(ident) => write!(f, "Cannot mutate immutable variable '{ident}'"),
		}
	}
}
