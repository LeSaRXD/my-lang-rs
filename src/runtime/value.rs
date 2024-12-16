use std::{
	cell::{Ref, RefCell},
	fmt::{Debug, Display},
	ops::{Add, Div, Mul, Neg, Rem, Sub},
	rc::Rc,
};

use crate::numeric::Numeric;

use super::{inner_value::InnerRuntimeValue, RuntimeResult};

pub trait Pos {
	type Output;
	fn pos(self) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeValue {
	inner: Rc<RefCell<InnerRuntimeValue>>,
	pub mutable: bool,
}

impl RuntimeValue {
	fn new(inner: InnerRuntimeValue) -> Self {
		Self {
			inner: Rc::new(RefCell::new(inner)),
			mutable: false,
		}
	}

	pub fn inner(&self) -> Ref<'_, InnerRuntimeValue> {
		self.inner.borrow()
	}

	pub fn same_type(&self, other: &Self) -> bool {
		self.inner().discriminant() == other.inner().discriminant()
	}

	pub fn number(number: Numeric) -> Self {
		Self::new(InnerRuntimeValue::Number(number))
	}

	pub fn string(string: String) -> Self {
		Self::new(InnerRuntimeValue::String(string))
	}

	pub fn unit() -> Self {
		Self::new(InnerRuntimeValue::Unit)
	}
}

impl Display for RuntimeValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			if self.mutable { "~" } else { "" },
			&self.inner()
		)
	}
}

impl Pos for RuntimeValue {
	type Output = RuntimeResult;
	fn pos(self) -> Self::Output {
		self.inner().pos().map(RuntimeValue::new)
	}
}

impl Neg for RuntimeValue {
	type Output = RuntimeResult;
	fn neg(self) -> Self::Output {
		self.inner().neg().map(RuntimeValue::new)
	}
}

impl Add<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn add(self, rhs: RuntimeValue) -> Self::Output {
		self.inner().add(&rhs.inner()).map(RuntimeValue::new)
	}
}

impl Sub<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn sub(self, rhs: RuntimeValue) -> Self::Output {
		self.inner().sub(&rhs.inner()).map(RuntimeValue::new)
	}
}

impl Mul<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn mul(self, rhs: RuntimeValue) -> Self::Output {
		self.inner().mul(&rhs.inner()).map(RuntimeValue::new)
	}
}

impl Div<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn div(self, rhs: RuntimeValue) -> Self::Output {
		self.inner().div(&rhs.inner()).map(RuntimeValue::new)
	}
}

impl Rem<RuntimeValue> for RuntimeValue {
	type Output = RuntimeResult;
	fn rem(self, rhs: RuntimeValue) -> Self::Output {
		self.inner().rem(&rhs.inner()).map(RuntimeValue::new)
	}
}
