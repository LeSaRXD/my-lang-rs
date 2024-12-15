use std::fmt::Display;

use crate::ast::{binary_expr::BinaryOp, unary_expr::UnaryOp};

use super::inner_value::InnerRuntimeValue;

#[derive(Debug, Clone)]
pub enum RuntimeOperation {
	Unary(UnaryOp, InnerRuntimeValue),
	Binary(InnerRuntimeValue, BinaryOp, InnerRuntimeValue),
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
}

impl Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RuntimeError::*;

		match self {
			UnsupportedOperation(op) => write!(f, "Unsupported operation: {op}"),
			VariableNotDeclared(varname) => write!(f, "Variable '{varname}' is not declared"),
			VariableTypeDoesntMatch(varname) => {
				write!(f, "Variable '{varname}' is of a different type")
			}
		}
	}
}
