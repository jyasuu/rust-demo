use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // Worker thread
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
        println!("Condition met, notifying main thread...");
    });

    // Main thread
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }

    println!("Condition met, proceeding...");
    handle.join().unwrap();
}


#![allow(unused)]
fn main_doc() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    println!("1");
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        println!("2");
        thread::sleep(std::time::Duration::from_millis(100));
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("3");
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
        println!("4");
    });
    
    println!("5");
    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("6");
    while !*started {
        println!("7");
        started = cvar.wait(started).unwrap();
        println!("8");
    }
    println!("9");
}


// 1
// 5
// 6
// 7
// 2
// 3
// 4
// 8
// 9

