use std::{
	cell::{Ref, RefCell, RefMut},
	collections::HashMap,
	rc::Rc,
};

use crate::runtime::{error::RuntimeError::*, value::RuntimeValue, RuntimeResult};

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

	pub fn assign(&self, ident: &str, mut value: RuntimeValue) -> RuntimeResult {
		let mut inner = self.inner_mut();
		if let Some(old) = inner.variables.get_mut(ident) {
			match (old.mutable, old.same_type(&value)) {
				(true, true) => {
					value.mutable = true;
					*old = value;
					Ok(old.to_owned())
				}
				(true, false) => Err(VariableTypeDoesntMatch(Box::from(ident))),
				(false, _) => Err(CannotMutateVariable(Box::from(ident))),
			}
		} else if let Some(parent) = &inner.parent {
			parent.assign(ident, value)
		} else {
			Err(VariableNotDeclared(Box::from(ident)))
		}
	}

	pub fn evaluate(&self, ident: &str) -> RuntimeResult {
		let inner = self.inner();

		match inner.variables.get(ident) {
			Some(value) => Ok(value.to_owned()),
			None => match &inner.parent {
				Some(p) => p.evaluate(ident),
				None => Err(VariableNotDeclared(Box::from(ident))),
			},
		}
	}
}
