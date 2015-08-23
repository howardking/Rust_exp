use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
	let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

	for i in 0..3{
		let data = data.clone();
		thread::spawn(move||{
			let mut data = data.lock().unwrap();
			data[i] += 1;
		});
	}

	
	thread::sleep_ms(50);

	let handle = thread::spawn(||{
		println!("This is in a thread, not main");
		thread::sleep_ms(5000);
		"return from thread"
	});

	thread::sleep_ms(5000);
	println!("{}",handle.join().unwrap());
	thread::sleep_ms(5000);

    println!("Hello, world, in main thread!");
	println!("-------------------------------");

	let (tx, rx) = mpsc::channel();

	for _ in 0..10{
		let tx = tx.clone();

		thread::spawn(move ||{
			let answer = 42u32;

			tx.send(answer);
		});
	}

	let mut receive = rx.recv().ok().expect("Could not receive answer");
	println!("{}",receive);
}
