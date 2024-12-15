use std::{
	cell::{Ref, RefCell, RefMut},
	collections::HashMap,
	rc::Rc,
};

use crate::runtime::{error::RuntimeError, value::RuntimeValue, RuntimeResult};

struct InnerEnv {
	parent: Option<Env>,
	variables: HashMap<Box<str>, RuntimeValue>,
}
pub struct Env {
	inner: Rc<RefCell<InnerEnv>>,
}

impl Clone for Env {
	fn clone(&self) -> Self {
		Self {
			inner: Rc::clone(&self.inner),
		}
	}
}

impl Env {
	fn inner_mut(&self) -> RefMut<'_, InnerEnv> {
		self.inner.borrow_mut()
	}
	fn inner(&self) -> Ref<'_, InnerEnv> {
		self.inner.borrow()
	}

	fn new_with_parent(parent: Option<Self>) -> Self {
		let inner = Rc::new(RefCell::new(InnerEnv {
			parent,
			variables: HashMap::new(),
		}));
		Self { inner }
	}

	pub fn global() -> Self {
		Self::new_with_parent(None)
	}

	pub fn new(parent: Self) -> Self {
		Self::new_with_parent(Some(parent))
	}

	pub fn declare(&self, ident: &str, value: RuntimeValue) -> RuntimeValue {
		self.inner_mut()
			.variables
			.insert(Box::from(ident), value.clone());
		value
	}

	pub fn assign(&self, ident: &str, value: RuntimeValue) -> RuntimeResult {
		match self.inner_mut().variables.get_mut(ident) {
			Some(old) => {
				if old.same_type(&value) {
					Ok(self.declare(ident, value))
				} else {
					Err(RuntimeError::VariableTypeDoesntMatch(Box::from(ident)))
				}
			}
			None => match &self.inner_mut().parent {
				Some(parent) => parent.assign(ident, value),
				None => Err(RuntimeError::VariableNotDeclared(Box::from(ident))),
			},
		}
	}

	pub fn evaluate(&self, ident: &str) -> Result<RuntimeValue, RuntimeError> {
		let inner = self.inner();

		match inner.variables.get(ident) {
			Some(value) => Ok(value.to_owned()),
			None => match &inner.parent {
				Some(p) => p.evaluate(ident),
				None => Err(RuntimeError::VariableNotDeclared(Box::from(ident))),
			},
		}
	}
}
