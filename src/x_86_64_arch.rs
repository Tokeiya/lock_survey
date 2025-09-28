use crate::WRITER;
use crate::env_reporter::get_temp;
use std::hint::spin_loop;
use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

#[repr(align(128))]
struct Data(AtomicUsize);

struct Result(AtomicBool);

static X: Data = Data(AtomicUsize::new(0));
static Y: Data = Data(AtomicUsize::new(0));
static BARRIER: Barrier = Barrier::new(3);

static X_RESULT: Result = Result(AtomicBool::new(false));
static Y_RESULT: Result = Result(AtomicBool::new(false));

static ORDERED: AtomicUsize = AtomicUsize::new(0);
static UNORDERED: AtomicUsize = AtomicUsize::new(0);

const PERIOD: usize = 0x7f_ff;
const SIZE: usize = PERIOD * 100;

pub fn run() {
	let x = thread::spawn(proc_a);
	let y = thread::spawn(proc_b);
	let o = thread::spawn(observe);

	x.join().unwrap();
	y.join().unwrap();
	o.join().unwrap();
}

fn observe() {
	for i in 0..SIZE {
		X.0.store(0, Ordering::Release);
		Y.0.store(0, Ordering::Release);
		X_RESULT.0.store(false, Ordering::Release);
		Y_RESULT.0.store(false, Ordering::Release);
		BARRIER.wait();

		if i & PERIOD == 0 {
			let tmp = get_temp().unwrap_or(0.0);
			let ordered = ORDERED.load(Ordering::Acquire);
			let unordered = UNORDERED.load(Ordering::Acquire);

			println!(
				"{tmp:.2} {i}/{SIZE} {:.2}% ordered:{ordered} unordered:{unordered}",
				i as f64 / SIZE as f64 * 100.0
			);

			#[rustfmt::skip]
			WRITER.write(format!(r#"{{"count":{i},"cat":"summary","type":"x86_64","temp":{tmp:.2},"ordered":{ordered},"unordered":{unordered}}}
"#)).unwrap();
		}

		BARRIER.wait();

		if X_RESULT.0.load(Ordering::SeqCst) && Y_RESULT.0.load(Ordering::SeqCst) {
			UNORDERED.fetch_add(1, Ordering::Relaxed);

			let tmp = get_temp().unwrap_or(0.0);
			let ordered = ORDERED.load(Ordering::Relaxed);
			let unordered = UNORDERED.load(Ordering::Relaxed);

			#[rustfmt::skip]
			WRITER.write(format!(r#"{{"count":{i},"cat":"immd","type":"x86_64","temp":{tmp:.2},"ordered":{ordered},"unordered":{unordered}}}
"#)).unwrap();
		} else {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		}
	}

	let tmp = get_temp().unwrap_or(0.0);
	let ordered = ORDERED.load(Ordering::Acquire);
	let unordered = UNORDERED.load(Ordering::Acquire);

	#[rustfmt::skip]
			WRITER.write(format!(r#"{{"count":{SIZE},"cat":"summary","type":"intel","temp":{tmp:.2},"ordered":{ordered},"unordered":{unordered}}}
"#)).unwrap();
}

fn proc_a() {
	for _ in 0..SIZE {
		while X.0.load(Ordering::Acquire) != 0 || Y.0.load(Ordering::Acquire) != 0 {
			spin_loop()
		}

		while X_RESULT.0.load(Ordering::Acquire) || Y_RESULT.0.load(Ordering::Acquire) {
			spin_loop()
		}
		BARRIER.wait();
		X.0.store(1, Ordering::Relaxed);
		// fence(Ordering::SeqCst);
		if Y.0.load(Ordering::Relaxed) == 0 {
			X_RESULT.0.store(true, Ordering::Relaxed);
		}
		BARRIER.wait();
	}
}

fn proc_b() {
	for _ in 0..SIZE {
		while X.0.load(Ordering::Acquire) != 0 || Y.0.load(Ordering::Acquire) != 0 {
			spin_loop()
		}

		while X_RESULT.0.load(Ordering::Acquire) || Y_RESULT.0.load(Ordering::Acquire) {
			spin_loop()
		}

		BARRIER.wait();
		Y.0.store(1, Ordering::Relaxed);
		// fence(Ordering::SeqCst);
		if X.0.load(Ordering::Relaxed) == 0 {
			Y_RESULT.0.store(true, Ordering::Relaxed);
		}
		BARRIER.wait();
	}
}
