use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::{Read, Result as IoResult, Seek, SeekFrom};
use std::str::FromStr;

pub fn get_temp() -> IoResult<f64> {
	thread_local! {
	 static FILE:RefCell<IoResult<File>> =RefCell::new(File::open("/sys/class/thermal/thermal_zone0/temp"));
	}

	FILE.with_borrow_mut(|file| match file {
		Ok(x) => {
			let mut buff = String::new();
			_ = x.seek(SeekFrom::Start(0));
			_ = x.read_to_string(&mut buff)?;
			Ok(f64::from_str(buff.trim()).unwrap() / 1000.0)
		}
		Err(err) => return Err(io::Error::new(err.kind(), err.to_string())),
	})
}
