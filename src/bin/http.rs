use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("3.224.80.105:80")?;

    // Construct the HTTP GET request
    let request = "GET / HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Print the response
    println!("{}", response);

    Ok(())
}
