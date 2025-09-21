use crate::WRITER;
use std::hint::spin_loop;
use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

#[repr(align(128))]
struct Data(AtomicUsize);
#[repr(align(128))]
struct Flag(AtomicBool);
static BARRIER: Barrier = Barrier::new(4);
static DATA: Data = Data(AtomicUsize::new(0));
static FLG: Flag = Flag(AtomicBool::new(false));
static ORDERED: AtomicUsize = AtomicUsize::new(0);
static UNORDERED: AtomicUsize = AtomicUsize::new(0);
const SIZE: usize = 10_000_000;

pub fn run() {
	let release_thread = thread::spawn(|| release());
	let sub_acquire_thread = thread::spawn(|| acquire(1));
	let acquire_thread1 = thread::spawn(|| sub_acquire(2));
	let acquire_thread2 = thread::spawn(|| sub_acquire(3));
	release_thread.join().unwrap();
	sub_acquire_thread.join().unwrap();
	acquire_thread1.join().unwrap();
	acquire_thread2.join().unwrap();
	println!("done");
}

fn release() {
	for i in 0..SIZE {
		BARRIER.wait();
		thread::yield_now();
		DATA.0.store(i, Ordering::Relaxed);
		FLG.0.store(true, Ordering::Relaxed);
		BARRIER.wait();
	}
}
fn sub_acquire(id: usize) {
	for i in 0..SIZE {
		BARRIER.wait();
		while !FLG.0.load(Ordering::Relaxed) {
			spin_loop();
		}
		let observed = DATA.0.load(Ordering::Relaxed);
		if observed == i {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		} else {
			WRITER
				.write(format!("{},{},{},{}\n", id, SIZE, i, observed))
				.unwrap();
			UNORDERED.fetch_add(1, Ordering::Relaxed);
		}
		BARRIER.wait();
	}
}
fn acquire(id: usize) {
	for i in 0..SIZE {
		BARRIER.wait();
		while !FLG.0.load(Ordering::Relaxed) {
			spin_loop();
		}
		let observed = DATA.0.load(Ordering::Relaxed);
		if observed == i {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		} else {
			WRITER
				.write(format!("{},{},{},{}\n", id, SIZE, i, observed))
				.unwrap();
			UNORDERED.fetch_add(1, Ordering::Relaxed);
		}
		if i & 0x7fff == 0 {
			println!(
				"{}/{} {:.2}% ordered:{} unordered:{}",
				i,
				SIZE,
				i as f64 / SIZE as f64 * 100.0,
				ORDERED.load(Ordering::Relaxed),
				UNORDERED.load(Ordering::Relaxed)
			);
		}
		BARRIER.wait();
		FLG.0.store(false, Ordering::Relaxed);
	}
}
