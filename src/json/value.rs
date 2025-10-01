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

fn is_contained_escape_char(s: &str) -> bool {
	for elem in s.as_bytes() {
		if elem & 0x80 != 0 {
			continue;
		}

		match elem {
			b'\"' => return true,
			b'\\' => return true,
			0x08 => return true, //backspace
			0x0C => return true, //form feed
			b'\n' => return true,
			b'\r' => return true,
			b'\t' => return true,
			_ => continue,
		}
	}

	false
}

fn escaped_string(scr: &str) -> Cow<str> {
	if is_contained_escape_char(scr) {
		let mut str = Vec::new();

		for elem in scr.as_bytes() {
			if elem & 0x80 != 0 {
				str.push(*elem);
			} else {
				match elem {
					b'\"' => {
						str.push(b'\\');
						str.push(b'\"');
					}
					b'\\' => {
						str.push(b'\\');
						str.push(b'\\');
					}
					0x08 => {
						str.push(b'\\');
						str.push(b'b');
					} //backspace
					0x0C => {
						str.push(b'\\');
						str.push(b'f');
					} //form feed
					b'\n' => {
						str.push(b'\\');
						str.push(b'n');
					}
					b'\r' => {
						str.push(b'\\');
						str.push(b'r');
					}
					b'\t' => {
						str.push(b'\\');
						str.push(b't');
					}
					_ => {
						str.push(*elem);
					}
				}
			}
		}
		Cow::Owned(String::from_utf8(str).unwrap())
	} else {
		Cow::Borrowed(scr)
	}
}

fn array_to_str(arr: &[Value]) -> String {
	let mut str = String::new();

	for elem in arr {
		str.push_str(&elem.to_string());
		str.push(',');
	}

	str.pop();
	str
}

fn object_to_str(obj: &HashMap<String, Value>) -> String {
	let mut str = String::new();

	for (k, v) in obj.iter() {
		str.push('"');
		str.push_str(&escaped_string(k));
		str.push('"');
		str.push(':');
		str.push_str(&v.to_string());
		str.push(',');
	}

	str.pop();
	str
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Null => write!(f, "null"),
			Value::True => write!(f, "true"),
			Value::False => write!(f, "false"),
			Value::String(x) => write!(f, "\"{}\"", escaped_string(x)),
			Value::Number(x) => write!(f, "{}", x),
			Value::Array(x) => write!(f, "[{}]", array_to_str(x)),
			Value::Object(x) => write!(f, "{{{}}}", object_to_str(x)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn array_to_str_test() {
		let vec: Vec<Value> = vec![];
		assert_eq!(array_to_str(&vec), "");
		let fixture = Value::from(vec);
		assert_eq!(fixture.to_string(), "[]");

		let mut vec: Vec<Value> = vec![Value::from("hello")];
		assert_eq!(array_to_str(&vec), "\"hello\"");

		vec.push(Value::from(100));
		assert_eq!(array_to_str(&vec), "\"hello\",100");

		vec.push(Value::from(42.195));
		assert_eq!(array_to_str(&vec), "\"hello\",100,42.195");

		vec.push(Value::True);
		assert_eq!(array_to_str(&vec), "\"hello\",100,42.195,true");

		vec.push(Value::False);
		assert_eq!(array_to_str(&vec), "\"hello\",100,42.195,true,false");

		vec.push(Value::Null);
		assert_eq!(array_to_str(&vec), "\"hello\",100,42.195,true,false,null");

		let fixture = Value::from(vec);
		assert_eq!(
			fixture.to_string(),
			"[\"hello\",100,42.195,true,false,null]"
		);
	}

	#[test]
	fn object_to_str_test() {
		todo!();
	}

	#[test]
	fn escape_str_test() {
		assert_eq!(escaped_string("hello\"world"), "hello\\\"world");
		assert_eq!(escaped_string("hello\\world"), "hello\\\\world");

		let mut vec: Vec<u8> = Vec::new();

		for elem in b"hello" {
			vec.push(*elem);
		}
		vec.push(0x08);

		for elem in b"world" {
			vec.push(*elem);
		}

		assert_eq!(
			escaped_string(String::from_utf8(vec).unwrap().as_str()),
			"hello\\bworld"
		);

		let mut vec: Vec<u8> = Vec::new();
		for elem in b"hello" {
			vec.push(*elem);
		}

		vec.push(0x0C);

		for elem in b"world" {
			vec.push(*elem);
		}

		assert_eq!(
			escaped_string(String::from_utf8(vec).unwrap().as_str()),
			"hello\\fworld"
		);

		assert_eq!(escaped_string("hello\nworld"), "hello\\nworld");
		assert_eq!(escaped_string("hello\rworld"), "hello\\rworld");
		assert_eq!(escaped_string("hello\tworld"), "hello\\tworld");

		assert_eq!(escaped_string("hello world"), "hello world");
		assert_eq!(escaped_string("今日は世界"), "今日は世界");

		assert_eq!(escaped_string("今日は\\世界"), "今日は\\\\世界");
	}

	#[test]
	fn is_contained_escaped_char_test() {
		assert!(is_contained_escape_char("\""));
		assert!(is_contained_escape_char("\\"));
		assert!(is_contained_escape_char(
			String::from_utf8(vec![0x08]).unwrap().as_str()
		));
		assert!(is_contained_escape_char(
			String::from_utf8(vec![0x0C]).unwrap().as_str()
		));
		assert!(is_contained_escape_char("\n"));
		assert!(is_contained_escape_char("\r"));
		assert!(is_contained_escape_char("\t"));

		assert!(!is_contained_escape_char("hello world"));
	}
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
