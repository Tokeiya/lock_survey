use std::hint::spin_loop;
use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, fence};
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

const SIZE: usize = 1_000_000;

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
		if i & 0x07fff == 0 {
			println!(
				"i:{} ORDERD:{} UNORDERED:{}",
				i,
				ORDERED.load(Ordering::Relaxed),
				UNORDERED.load(Ordering::Relaxed)
			);
		}
		BARRIER.wait();
		if X_RESULT.0.load(Ordering::SeqCst) && Y_RESULT.0.load(Ordering::SeqCst) {
			UNORDERED.fetch_add(1, Ordering::Relaxed);
		} else {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		}
	}
}

//noinspection DuplicatedCode
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

		fence(Ordering::SeqCst);

		if Y.0.load(Ordering::Relaxed) == 0 {
			X_RESULT.0.store(true, Ordering::SeqCst);
		}
		BARRIER.wait();
	}
}

//noinspection DuplicatedCode
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

		fence(Ordering::SeqCst);

		if X.0.load(Ordering::Relaxed) == 0 {
			Y_RESULT.0.store(true, Ordering::SeqCst);
		}
		BARRIER.wait();
	}
}
