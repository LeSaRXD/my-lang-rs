use std::{
	fmt::Display,
	ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Numeric {
	Int(i128),
	Float(f64),
}

impl Eq for Numeric {}

impl Add<Numeric> for Numeric {
	type Output = Self;
	fn add(self, rhs: Numeric) -> Self::Output {
		use Numeric::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 + i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) + f2),
			(Float(f1), Int(i2)) => Float(f1 + (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 + f2),
		}
	}
}

impl Sub<Numeric> for Numeric {
	type Output = Self;
	fn sub(self, rhs: Numeric) -> Self::Output {
		use Numeric::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 - i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) - f2),
			(Float(f1), Int(i2)) => Float(f1 - (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 - f2),
		}
	}
}

impl Neg for Numeric {
	type Output = Self;
	fn neg(self) -> Self::Output {
		use Numeric::*;
		match self {
			Int(i) => Int(-i),
			Float(f) => Float(-f),
		}
	}
}

impl Mul<Numeric> for Numeric {
	type Output = Self;
	fn mul(self, rhs: Numeric) -> Self::Output {
		use Numeric::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 * i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) * f2),
			(Float(f1), Int(i2)) => Float(f1 * (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 * f2),
		}
	}
}

impl Div<Numeric> for Numeric {
	type Output = Self;
	fn div(self, rhs: Numeric) -> Self::Output {
		use Numeric::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 / i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) / f2),
			(Float(f1), Int(i2)) => Float(f1 / (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 / f2),
		}
	}
}

impl Rem<Numeric> for Numeric {
	type Output = Numeric;
	fn rem(self, rhs: Numeric) -> Self::Output {
		use Numeric::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 % i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) % f2),
			(Float(f1), Int(i2)) => Float(f1 % (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 % f2),
		}
	}
}

impl Display for Numeric {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Numeric::*;

		match self {
			Int(i) => f.write_str(&i.to_string()),
			Float(fl) => f.write_str(&fl.to_string()),
		}
	}
}
