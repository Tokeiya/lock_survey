mod aarch;
mod channel_data;
mod dummy_data;
mod env_reporter;
mod interl_arch;
mod writer;
mod writer_error;

use crate::env_reporter::get_temp;
use crate::writer::Writer;
use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::LazyLock;

fn create_file_data(path: &str) -> File {
	let now = Local::now();
	let a = now.format("%y_%m_%d_%H_%M.txt").to_string();

	File::create(format!("{}{}", path, a)).unwrap()
}

pub static WRITER: LazyLock<Writer> =
	LazyLock::new(move || Writer::new(100, create_file_data("./foo")));

fn main() {
	let mut buf = String::new();

	println!("{:?}", get_temp());
	std::thread::sleep(std::time::Duration::from_secs(1));
	println!("{:?}", get_temp());

	//aarch::run();
}
