use std::collections::HashMap;
use std::io::{self, Read, Write, ErrorKind};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::time::{Duration, Instant};

// Simple non-blocking TCP server
struct NonBlockingServer {
    listener: TcpListener,
    clients: HashMap<SocketAddr, TcpStream>,
}

impl NonBlockingServer {
    fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        println!("Server listening on {}", addr);
        
        Ok(NonBlockingServer {
            listener,
            clients: HashMap::new(),
        })
    }

    fn run(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1024];
        
        loop {
            // Try to accept new connections (non-blocking)
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    println!("New client connected: {}", addr);
                    stream.set_nonblocking(true)?;
                    self.clients.insert(addr, stream);
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // No new connections, continue to handle existing clients
                }
                Err(e) => return Err(e),
            }

            // Handle existing clients
            let mut disconnected = Vec::new();
            
            for (addr, stream) in &mut self.clients {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        // Client disconnected
                        println!("Client {} disconnected", addr);
                        disconnected.push(*addr);
                    }
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received from {}: {}", addr, message.trim());
                        
                        // Echo the message back
                        let response = format!("Echo: {}", message);
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            println!("Failed to write to {}: {}", addr, e);
                            disconnected.push(*addr);
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // No data available, continue with next client
                    }
                    Err(e) => {
                        println!("Error reading from {}: {}", addr, e);
                        disconnected.push(*addr);
                    }
                }
            }

            // Remove disconnected clients
            for addr in disconnected {
                self.clients.remove(&addr);
            }

            // Small delay to prevent busy waiting
            thread::sleep(Duration::from_millis(10));
        }
    }
}

// Non-blocking TCP client
struct NonBlockingClient {
    stream: TcpStream,
    last_send: Instant,
}

impl NonBlockingClient {
    fn new(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        println!("Connected to server at {}", addr);
        
        Ok(NonBlockingClient {
            stream,
            last_send: Instant::now(),
        })
    }

    fn run(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1024];
        let mut message_count = 0;

        loop {
            // Send a message every 2 seconds
            if self.last_send.elapsed() >= Duration::from_secs(2) {
                message_count += 1;
                let message = format!("Hello from client #{}\n", message_count);
                
                match self.stream.write_all(message.as_bytes()) {
                    Ok(_) => {
                        println!("Sent: {}", message.trim());
                        self.last_send = Instant::now();
                    }
                    Err(e) => {
                        println!("Failed to send message: {}", e);
                        return Err(e);
                    }
                }
            }

            // Try to read response (non-blocking)
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server disconnected");
                    break;
                }
                Ok(n) => {
                    let response = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received: {}", response.trim());
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // No data available, continue
                }
                Err(e) => {
                    println!("Error reading: {}", e);
                    return Err(e);
                }
            }

            // Stop after sending 5 messages
            if message_count >= 5 {
                println!("Client finished sending messages");
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}

// Event-driven approach using mio (commented out as it requires external crate)
/*
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

const SERVER_TOKEN: Token = Token(0);
const CLIENT_TOKEN: Token = Token(1);

struct EventDrivenServer {
    poll: Poll,
    listener: TcpListener,
    events: Events,
}

impl EventDrivenServer {
    fn new(addr: &str) -> io::Result<Self> {
        let mut poll = Poll::new()?;
        let mut listener = TcpListener::bind(addr.parse().unwrap())?;
        
        poll.registry().register(
            &mut listener,
            SERVER_TOKEN,
            Interest::READABLE,
        )?;

        Ok(EventDrivenServer {
            poll,
            listener,
            events: Events::with_capacity(1024),
        })
    }

    fn run(&mut self) -> io::Result<()> {
        loop {
            self.poll.poll(&mut self.events, None)?;

            for event in &self.events {
                match event.token() {
                    SERVER_TOKEN => {
                        // Accept new connections
                        loop {
                            match self.listener.accept() {
                                Ok((stream, addr)) => {
                                    println!("New connection: {}", addr);
                                    // Register new client...
                                }
                                Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    // Handle client events...
                    _ => {}
                }
            }
        }
    }
}
*/

// Manual event loop simulation
struct SimpleEventLoop {
    events: Vec<Box<dyn Event>>,
}

trait Event {
    fn handle(&mut self) -> io::Result<bool>; // returns true if event should continue
    fn name(&self) -> &str;
}

struct TimerEvent {
    name: String,
    interval: Duration,
    last_trigger: Instant,
    count: u32,
    max_count: u32,
}

impl TimerEvent {
    fn new(name: &str, interval: Duration, max_count: u32) -> Self {
        TimerEvent {
            name: name.to_string(),
            interval,
            last_trigger: Instant::now(),
            count: 0,
            max_count,
        }
    }
}

impl Event for TimerEvent {
    fn handle(&mut self) -> io::Result<bool> {
        if self.last_trigger.elapsed() >= self.interval {
            self.count += 1;
            println!("{}: Tick #{}", self.name, self.count);
            self.last_trigger = Instant::now();
            
            if self.count >= self.max_count {
                println!("{}: Finished", self.name);
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl SimpleEventLoop {
    fn new() -> Self {
        SimpleEventLoop {
            events: Vec::new(),
        }
    }

    fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }

    fn run(&mut self) -> io::Result<()> {
        println!("Starting event loop...");
        
        while !self.events.is_empty() {
            let mut to_remove = Vec::new();
            
            for (i, event) in self.events.iter_mut().enumerate() {
                match event.handle() {
                    Ok(true) => continue, // Event continues
                    Ok(false) => to_remove.push(i), // Event finished
                    Err(e) => {
                        println!("Event '{}' error: {}", event.name(), e);
                        to_remove.push(i);
                    }
                }
            }

            // Remove finished events (in reverse order to maintain indices)
            for &i in to_remove.iter().rev() {
                self.events.remove(i);
            }

            thread::sleep(Duration::from_millis(50));
        }

        println!("Event loop finished");
        Ok(())
    }
}

fn main() -> io::Result<()> {
    println!("=== Rust Non-blocking Socket Practice ===\n");

    // Demo 1: Simple event loop
    println!("1. Simple Event Loop Demo:");
    let mut event_loop = SimpleEventLoop::new();
    event_loop.add_event(Box::new(TimerEvent::new("Fast Timer", Duration::from_millis(500), 3)));
    event_loop.add_event(Box::new(TimerEvent::new("Slow Timer", Duration::from_secs(1), 2)));
    event_loop.run()?;

    println!("\n2. Socket Demo:");
    println!("To test the socket code, run in separate terminals:");
    println!("   - Server: cargo run -- server");
    println!("   - Client: cargo run -- client");
    
    // Parse command line arguments for socket demo
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "server" => {
                let mut server = NonBlockingServer::new("127.0.0.1:8080")?;
                server.run()?;
            }
            "client" => {
                thread::sleep(Duration::from_millis(100)); // Give server time to start
                let mut client = NonBlockingClient::new("127.0.0.1:8080")?;
                client.run()?;
            }
            _ => {
                println!("Usage: cargo run -- [server|client]");
            }
        }
    } else {
        println!("Run with 'server' or 'client' argument to test sockets");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_timer_event() {
        let mut timer = TimerEvent::new("test", Duration::from_millis(1), 2);
        
        // Should not trigger immediately
        assert!(timer.handle().unwrap());
        assert_eq!(timer.count, 0);
        
        // Wait and trigger
        thread::sleep(Duration::from_millis(2));
        assert!(timer.handle().unwrap());
        assert_eq!(timer.count, 1);
        
        // Should finish after max_count
        thread::sleep(Duration::from_millis(2));
        assert!(!timer.handle().unwrap());
        assert_eq!(timer.count, 2);
    }

    #[test]
    fn test_event_loop() {
        let mut event_loop = SimpleEventLoop::new();
        event_loop.add_event(Box::new(TimerEvent::new("test", Duration::from_millis(1), 1)));
        
        // Should complete without error
        assert!(event_loop.run().is_ok());
        assert!(event_loop.events.is_empty());
    }
}

// Additional practice exercises:

/*
EXERCISES TO TRY:

1. Modify the server to broadcast messages to all connected clients
2. Add a heartbeat mechanism to detect disconnected clients
3. Implement a simple chat protocol with usernames
4. Add SSL/TLS support using rustls
5. Create a connection pool for the client
6. Implement rate limiting on the server
7. Add message framing (length-prefixed messages)
8. Create a pub/sub system with topics
9. Add authentication and authorization
10. Implement graceful shutdown handling

CONCEPTS COVERED:
- Non-blocking I/O with set_nonblocking(true)
- Error handling with ErrorKind::WouldBlock
- Event loops and polling
- Connection management
- Buffer handling
- TCP client/server patterns
- Thread-based concurrency
- Timer events
- Resource cleanup
*/



// # server
// === Rust Non-blocking Socket Practice ===

// 1. Simple Event Loop Demo:
// Starting event loop...
// Fast Timer: Tick #1
// Fast Timer: Tick #2
// Slow Timer: Tick #1
// Fast Timer: Tick #3
// Fast Timer: Finished
// Slow Timer: Tick #2
// Slow Timer: Finished
// Event loop finished

// 2. Socket Demo:
// To test the socket code, run in separate terminals:
//    - Server: cargo run -- server
//    - Client: cargo run -- client
// Server listening on 127.0.0.1:8080
// New client connected: 127.0.0.1:49000
// Received from 127.0.0.1:49000: Hello from client #1
// Received from 127.0.0.1:49000: Hello from client #2
// Received from 127.0.0.1:49000: Hello from client #3
// Received from 127.0.0.1:49000: Hello from client #4
// Received from 127.0.0.1:49000: Hello from client #5
// Client 127.0.0.1:49000 disconnected

// # client
// === Rust Non-blocking Socket Practice ===

// 1. Simple Event Loop Demo:
// Starting event loop...
// Fast Timer: Tick #1
// Fast Timer: Tick #2
// Slow Timer: Tick #1
// Fast Timer: Tick #3
// Fast Timer: Finished
// Slow Timer: Tick #2
// Slow Timer: Finished
// Event loop finished

// 2. Socket Demo:
// To test the socket code, run in separate terminals:
//    - Server: cargo run -- server
//    - Client: cargo run -- client
// Connected to server at 127.0.0.1:8080
// Sent: Hello from client #1
// Received: Echo: Hello from client #1
// Sent: Hello from client #2
// Received: Echo: Hello from client #2
// Sent: Hello from client #3
// Received: Echo: Hello from client #3
// Sent: Hello from client #4
// Received: Echo: Hello from client #4
// Sent: Hello from client #5
// Client finished sending messages