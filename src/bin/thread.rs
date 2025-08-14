use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);
    
    // thread::sleep(std::time::Duration::from_millis(100));

    println!("Hello from the main thread.");
    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}


// Hello from another thread!
// This is my thread id: ThreadId(3)
// Hello from another thread!
// This is my thread id: ThreadId(2)
// Hello from the main thread.
