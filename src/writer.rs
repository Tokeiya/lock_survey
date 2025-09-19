use crate::channel_data::ChannelData;
use std::io::Write as IoWrite;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, SendError, SyncSender, sync_channel};
use std::thread;
use std::thread::JoinHandle;

pub struct Writer {
	sender: SyncSender<ChannelData>,
	handle: Mutex<Option<JoinHandle<()>>>,
}

impl Writer {
	pub fn new<T: IoWrite + Send + Sync + 'static>(bound: usize, writer: T) -> Self {
		let (snd, rcv) = sync_channel::<ChannelData>(bound);
		let handle: JoinHandle<()> = thread::spawn(move || Self::write_proc(rcv, writer));

		let handle = Mutex::new(Some(handle));

		Self {
			sender: snd,
			handle,
		}
	}

	pub fn write_str(&self, value: &str) -> Result<(), SendError<ChannelData>> {
		self.sender.send(ChannelData::Write(value.to_string()))
	}

	pub fn write(&self, value: String) -> Result<(), SendError<ChannelData>> {
		self.sender.send(ChannelData::Write(value))
	}

	fn write_proc<T: IoWrite + Send + Sync + 'static>(rcv: Receiver<ChannelData>, mut wtr: T) {
		for datum in rcv.iter() {
			match datum {
				ChannelData::Write(value) => {
					wtr.write(value.as_bytes()).unwrap();
					wtr.flush().unwrap();
				}
				ChannelData::Terminate => break,
			}
		}

		wtr.flush().unwrap();
	}

	pub fn terminate(&self) {
		let result = self.handle.lock().unwrap().take();

		if let Some(handle) = result {
			self.sender.send(ChannelData::Terminate).unwrap();
			handle.join().unwrap();
		}
	}
}

impl Drop for Writer {
	fn drop(&mut self) {
		self.terminate();
	}
}
