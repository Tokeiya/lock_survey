use crate::WRITER;
use crate::env_reporter::get_temp;
use std::hint::{black_box, spin_loop};
use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

#[repr(align(128))]
struct Data(AtomicUsize);
#[repr(align(128))]
struct Flag(AtomicBool);
#[repr(align(128))]
struct Dummy(AtomicUsize);

static BARRIER: Barrier = Barrier::new(4);
static DATA: Data = Data(AtomicUsize::new(0));
static FLG: Flag = Flag(AtomicBool::new(false));
static ORDERED: AtomicUsize = AtomicUsize::new(0);
static UNORDERED: AtomicUsize = AtomicUsize::new(0);

const PERIOD: usize = 0x7f_ff;
const SIZE: usize = PERIOD * 100;

const RELEASE: Ordering = Ordering::Relaxed;
const ACQUIRE: Ordering = Ordering::Relaxed;

static DUMMY1: Dummy = Dummy(AtomicUsize::new(0));
static DUMMY2: Dummy = Dummy(AtomicUsize::new(0));

pub fn run() {
	let release_thread = thread::spawn(|| release());
	let sub_acquire_thread = thread::spawn(|| acquire(1));
	let acquire_thread1 = thread::spawn(|| acquire(2));
	let acquire_thread2 = thread::spawn(|| acquire(3));
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
		DUMMY1.0.store(i, RELEASE);
		DATA.0.store(i, RELEASE);
		DUMMY2.0.store(i, RELEASE);
		FLG.0.store(true, RELEASE);
		BARRIER.wait();

		if i & PERIOD == 0 {
			let tmp = get_temp().unwrap_or_else(|_| 0.0);
			let ordered = ORDERED.load(Ordering::Relaxed);
			let unordered = UNORDERED.load(Ordering::Relaxed);

			println!(
				"{:.2}℃  {}/{} {:.2}% ordered:{ordered} unordered:{unordered}",
				tmp,
				i,
				SIZE,
				i as f64 / SIZE as f64 * 100.0
			);

			#[rustfmt::skip]
			WRITER
				.write(format! {
					r#"{{cat:"summary",type:"aarch",temp:{tmp:.2},ordered:{ordered},unordered:{unordered}}}
"#})
				.unwrap();
		}
		FLG.0.store(false, Ordering::Relaxed);
	}

	let tmp = get_temp().unwrap_or_else(|_| 0.0);
	let ordered = ORDERED.load(Ordering::Relaxed);
	let unordered = UNORDERED.load(Ordering::Relaxed);

	println!(
		"{:.2}℃  {}/{} {:.2}% ordered:{ordered} unordered:{unordered}",
		tmp, SIZE, SIZE, 100.0
	);

	WRITER
		.write(format! {
				r#"{{cat:"summary",type:"aarch",temp:{tmp:.2},ordered:{ordered},unordered:{unordered}}}
		"#})
		.unwrap();
}
fn acquire(id: usize) {
	let mut tmp = 0f64;
	let mut flg = false;
	let mut accum = 0usize;

	for i in 0..SIZE {
		BARRIER.wait();
		while !FLG.0.load(ACQUIRE) {
			accum += DUMMY1.0.load(ACQUIRE);
			spin_loop();
		}
		accum += DUMMY2.0.load(ACQUIRE);
		let observed = DATA.0.load(ACQUIRE);
		if observed == i {
			ORDERED.fetch_add(1, Ordering::Relaxed);
		} else {
			flg = true;
			tmp = get_temp().unwrap_or_else(|_| 0.0);
			UNORDERED.fetch_add(1, Ordering::Relaxed);
		}
		BARRIER.wait();

		if flg {
			WRITER
				.write(format!(
					r#"{{"cat:"immd",type:"aarch","temp":{:.2},"id":{},"size":{},"i":{},"observed":{}}}
"#,
					tmp, id, SIZE, i, observed
				))
				.unwrap();

			flg = false;
		}
	}

	black_box(accum);
}
