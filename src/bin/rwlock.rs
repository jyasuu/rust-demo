use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    let mut handles = vec![];

    // Reader threads
    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_lock = data.read().unwrap();
            println!("Read value: {}", *read_lock);
        });
        handles.push(handle);
    }

    // Writer thread
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut write_lock = data.write().unwrap();
        *write_lock += 1;
        println!("Updated value: {}", *write_lock);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
