use std::{
	cell::{Ref, RefCell},
	fmt::{Debug, Display},
	ops::{Add, Div, Mul, Neg, Rem, Sub},
	rc::Rc,
};

use crate::numeric::Numeric;

use super::{value::RuntimeValue, RuntimeResult};

pub trait Pos {
	type Output;
	fn pos(self) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeVariable {
	value: Rc<RefCell<RuntimeValue>>,
	pub mutable: bool,
}

impl RuntimeVariable {
	fn new(inner: RuntimeValue) -> Self {
		Self {
			value: Rc::new(RefCell::new(inner)),
			mutable: false,
		}
	}

	pub fn inner(&self) -> Ref<'_, RuntimeValue> {
		self.value.borrow()
	}

	pub fn same_type(&self, other: &Self) -> bool {
		self.inner().discriminant() == other.inner().discriminant()
	}

	pub fn number(number: Numeric) -> Self {
		Self::new(RuntimeValue::Number(number))
	}

	pub fn string(string: String) -> Self {
		Self::new(RuntimeValue::String(string))
	}

	pub fn unit() -> Self {
		Self::new(RuntimeValue::Unit)
	}
}

impl Display for RuntimeVariable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			if self.mutable { "~" } else { "" },
			&self.inner()
		)
	}
}

impl Pos for RuntimeVariable {
	type Output = RuntimeResult;
	fn pos(self) -> Self::Output {
		self.inner().pos().map(RuntimeVariable::new)
	}
}

impl Neg for RuntimeVariable {
	type Output = RuntimeResult;
	fn neg(self) -> Self::Output {
		self.inner().neg().map(RuntimeVariable::new)
	}
}

impl Add<RuntimeVariable> for RuntimeVariable {
	type Output = RuntimeResult;
	fn add(self, rhs: RuntimeVariable) -> Self::Output {
		self.inner().add(&rhs.inner()).map(RuntimeVariable::new)
	}
}

impl Sub<RuntimeVariable> for RuntimeVariable {
	type Output = RuntimeResult;
	fn sub(self, rhs: RuntimeVariable) -> Self::Output {
		self.inner().sub(&rhs.inner()).map(RuntimeVariable::new)
	}
}

impl Mul<RuntimeVariable> for RuntimeVariable {
	type Output = RuntimeResult;
	fn mul(self, rhs: RuntimeVariable) -> Self::Output {
		self.inner().mul(&rhs.inner()).map(RuntimeVariable::new)
	}
}

impl Div<RuntimeVariable> for RuntimeVariable {
	type Output = RuntimeResult;
	fn div(self, rhs: RuntimeVariable) -> Self::Output {
		self.inner().div(&rhs.inner()).map(RuntimeVariable::new)
	}
}

impl Rem<RuntimeVariable> for RuntimeVariable {
	type Output = RuntimeResult;
	fn rem(self, rhs: RuntimeVariable) -> Self::Output {
		self.inner().rem(&rhs.inner()).map(RuntimeVariable::new)
	}
}
