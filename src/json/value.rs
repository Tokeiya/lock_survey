use crate::json::number::Number;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Value {
	Null,
	True,
	False,
	String(String),
	Number(Number),
	Array(Vec<Value>),
	Object(HashMap<String, Value>),
}

impl From<&'_ str> for Value {
	fn from(value: &str) -> Self {
		Value::String(value.into())
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}

impl From<i64> for Value {
	fn from(s: i64) -> Self {
		Value::Number(Number::from(s))
	}
}

impl From<f64> for Value {
	fn from(value: f64) -> Self {
		Value::Number(Number::Real(value))
	}
}

impl From<Vec<Value>> for Value {
	fn from(value: Vec<Value>) -> Self {
		Value::Array(value)
	}
}

impl From<HashMap<String, Value>> for Value {
	fn from(value: HashMap<String, Value>) -> Self {
		Value::Object(value)
	}
}

fn escaped_string(s: &str) -> Cow<'_, str> {
	todo!()
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Null => write!(f, "null"),
			Value::True => write!(f, "true"),
			Value::False => write!(f, "false"),
			Value::String(x) => todo!(),
			Value::Number(x) => write!(f, "{}", x),
			Value::Array(x) => {
				todo!()
			}
			Value::Object(x) => {
				todo!()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_str() {
		let fixture = Value::from("hello");

		match fixture {
			Value::String(x) => assert_eq!(x, "hello"),
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_string() {
		let fixture = Value::from("hello".to_string());

		match fixture {
			Value::String(str) => assert_eq!(str, "hello"),
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_i64() {
		let fixture = Value::from(123);

		match fixture {
			Value::Number(Number::Integer(x)) => assert_eq!(x, 123),
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_f64() {
		let fixture = Value::from(123.456);

		match fixture {
			Value::Number(Number::Real(x)) => assert_eq!(x, 123.456),
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_array() {
		let arr: Vec<Value> = vec![Value::from("hello"), Value::from(100)];

		let fixture = Value::from(arr);

		if let Value::Array(arr) = fixture {
			assert_eq!(arr.len(), 2);

			assert!(matches!(arr[0], Value::String(ref v) if v == "hello"));
			assert!(matches!(arr[1], Value::Number(Number::Integer(v)) if v == 100));
		} else {
			unreachable!();
		}
	}

	#[test]
	fn from_object() {
		let mut obj = HashMap::new();
		obj.insert("hello".to_string(), Value::String("world".to_string()));
		obj.insert("number".to_string(), Value::Number(Number::from(1)));

		let fixture = Value::from(obj);

		if let Value::Object(obj) = fixture {
			assert_eq!(obj.len(), 2);
		}
	}
}
