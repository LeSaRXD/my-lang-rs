use std::{
	fmt::Display,
	ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[derive(Debug, Clone)]
pub enum Number {
	Int(i128),
	Float(f64),
}

impl Add<Number> for Number {
	type Output = Self;
	fn add(self, rhs: Number) -> Self::Output {
		use Number::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 + i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) + f2),
			(Float(f1), Int(i2)) => Float(f1 + (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 + f2),
		}
	}
}

impl Sub<Number> for Number {
	type Output = Self;
	fn sub(self, rhs: Number) -> Self::Output {
		use Number::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 - i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) - f2),
			(Float(f1), Int(i2)) => Float(f1 - (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 - f2),
		}
	}
}

impl Neg for Number {
	type Output = Self;
	fn neg(self) -> Self::Output {
		use Number::*;
		match self {
			Int(i) => Int(-i),
			Float(f) => Float(-f),
		}
	}
}

impl Mul<Number> for Number {
	type Output = Self;
	fn mul(self, rhs: Number) -> Self::Output {
		use Number::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 * i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) * f2),
			(Float(f1), Int(i2)) => Float(f1 * (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 * f2),
		}
	}
}

impl Div<Number> for Number {
	type Output = Self;
	fn div(self, rhs: Number) -> Self::Output {
		use Number::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 / i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) / f2),
			(Float(f1), Int(i2)) => Float(f1 / (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 / f2),
		}
	}
}

impl Rem<Number> for Number {
	type Output = Number;
	fn rem(self, rhs: Number) -> Self::Output {
		use Number::*;
		match (self, rhs) {
			(Int(i1), Int(i2)) => Int(i1 % i2),
			(Int(i1), Float(f2)) => Float((i1 as f64) % f2),
			(Float(f1), Int(i2)) => Float(f1 % (i2 as f64)),
			(Float(f1), Float(f2)) => Float(f1 % f2),
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Number::*;

		match self {
			Int(i) => f.write_str(&i.to_string()),
			Float(fl) => f.write_str(&fl.to_string()),
		}
	}
}
