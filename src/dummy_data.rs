use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::hint::black_box;
use std::sync::atomic::AtomicUsize;

pub struct DummyData([AtomicUsize; 8]);

impl DummyData {
	pub fn new() -> Self {
		Self([
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
			AtomicUsize::new(0),
		])
	}

	pub fn dummy_write(&self) {
		for x in self.0.iter() {
			x.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		}
	}

	pub fn dummy_read(&self) -> usize {
		let mut accum = 0;

		for x in self.0.iter() {
			accum += x.load(std::sync::atomic::Ordering::Relaxed);
		}

		black_box(accum);
		accum
	}
}
