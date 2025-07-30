use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Create a channel with unbounded capacity
    let (tx, rx) = mpsc::channel();
    
    // Spawn producer thread
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            thread::sleep(Duration::from_millis(500)); // Simulate work
            tx.send(msg).expect("Failed to send message");
            println!("After send.");
        }
        // Sender drops here, closing the channel
    });

    let start_time = Instant::now();
    let timeout = Duration::from_secs(3);
    
    // Main thread processing loop
    loop {
        // Non-blocking receive attempt
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received: {}", msg);
                thread::sleep(Duration::from_millis(200));
                println!("Received after working: {}", msg);
            },
            Err(mpsc::TryRecvError::Empty) => {
                // Channel empty but not closed -> do other work
                println!("No message yet, doing other tasks...");
                thread::sleep(Duration::from_millis(200)); // Simulate work
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Channel closed. Exiting.");
                break;
            }
        }
        
        // Timeout check
        if start_time.elapsed() >= timeout {
            println!("Timeout reached. Exiting.");
            break;
        }
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8c51f0e6c8c61ca50d0e1493195c778

// No message yet, doing other tasks...
// No message yet, doing other tasks...
// No message yet, doing other tasks...
// After send.
// Received: Hello
// Received after working: Hello
// No message yet, doing other tasks...
// After send.
// Received: from
// Received after working: from
// No message yet, doing other tasks...
// No message yet, doing other tasks...
// After send.
// Received: the
// Received after working: the
// No message yet, doing other tasks...
// After send.
// Received: thread
// Received after working: thread
// Channel closed. Exiting.
