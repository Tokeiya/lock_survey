use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar};

static mut DATA: u64 = 0;

static FLAG: AtomicBool = AtomicBool::new(false);
static ORDERED: AtomicUsize = AtomicUsize::new(0);
static UNORDERED: AtomicUsize = AtomicUsize::new(0);
fn main() {
	for i in 0..100_000_000u64 {
		trial(i);

		if i & 0x7f_ff == 0 {
			println!("i:{i} {}%", i as f64 / 100_000_000f64 * 100f64);
		}
	}

	println!(
		"Ordered:{} Unordered:{}",
		ORDERED.load(Ordering::Relaxed),
		UNORDERED.load(Ordering::Relaxed)
	);
}

fn trial(val: u64) {
	let barrier_a = Arc::new(Barrier::new(2));
	let barrier_b = barrier_a.clone();

	let a = std::thread::spawn(move || {
		barrier_a.wait();
		unsafe {
			DATA = val;
		}

		FLAG.store(true, Ordering::Relaxed);
	});

	let b = std::thread::spawn(move || {
		barrier_b.wait();
		while !FLAG.load(Ordering::Relaxed) {}
		let loaded = unsafe { DATA };

		if loaded == val {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		} else {
			UNORDERED.fetch_add(1, Ordering::Relaxed);
			println!("Val:{} DATA:{} loaded:{}", val, unsafe { DATA }, loaded);
		}
	});

	a.join().unwrap();
	b.join().unwrap();

	FLAG.store(false, Ordering::Relaxed);
}
