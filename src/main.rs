mod aarch;
mod channel_data;
mod env_reporter;
mod json;
mod writer;
mod writer_error;
mod x_86_64_arch;

use crate::writer::Writer;
use chrono::Local;
use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::sync::LazyLock;

fn create_file_data(path: &str) -> File {
	let now = Local::now();
	println!("{now}");
	let a = now.format("%y_%m_%d_%H_%M.txt").to_string();

	File::create(format!("{}{}", path, a)).unwrap()
}

pub static WRITER: LazyLock<Writer> =
	LazyLock::new(move || Writer::new(100, create_file_data("./log")));

fn main() {
	static reg: LazyLock<regex::Regex> = LazyLock::new(|| Regex::new(r"[\\\x0c\x08\r\t]").unwrap());

	let s = r"\r";
	println!("{}", reg.is_match(s))
}

fn do_test() {
	let a: Cow<'static, str> = "hello".into();

	println!("aarch_64");
	aarch::run();

	println!("x86_64");
	x_86_64_arch::run();
}
