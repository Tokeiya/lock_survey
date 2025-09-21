use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::str::FromStr;
use std::sync::LazyLock;
pub fn get_temp() -> f64 {
	thread_local! {
	 static FILE:RefCell<File> =RefCell::new(File::open("/sys/class/thermal/thermal_zone0/temp").unwrap());
	}

	FILE.with_borrow_mut(|file| {
		let mut buff = String::new();
		_ = file.seek(SeekFrom::Start(0));
		file.read_to_string(&mut buff).unwrap();
		f64::from_str(buff.trim()).unwrap() / 1000.0
	})
}
