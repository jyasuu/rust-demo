use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(5));

    let mut handles = vec![];

    for i in 0..5 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} is waiting at the barrier...", i);
            barrier.wait();
            println!("Thread {} passed the barrier!", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
