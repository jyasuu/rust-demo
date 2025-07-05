// Link against the compiled C library
#[link(name = "example", kind = "static")]
extern "C" {
    fn print_from_c();
}

fn main() {
    println!("Hello from Rust!");
    
    unsafe {
        print_from_c();  // Call C function
    }
}