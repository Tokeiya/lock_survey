use std::fmt::Display;

#[derive(Debug)]
pub enum Number {
	Integer(i64),
	Real(f64),
}

impl From<i64> for Number {
	fn from(value: i64) -> Self {
		Number::Integer(value)
	}
}

impl From<f64> for Number {
	fn from(value: f64) -> Self {
		Number::Real(value)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Integer(x) => write!(f, "{}", x),
			Number::Real(x) => write!(f, "{}", x),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn from_i64() {
		let fixture = Number::from(123);
		match fixture {
			Number::Integer(x) => assert_eq!(x, 123),
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_f64() {
		let fixture = Number::from(123.456);
		match fixture {
			Number::Real(x) => assert_eq!(x, 123.456),
			_ => unreachable!(),
		}
	}
}
