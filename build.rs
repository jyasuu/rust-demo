fn main() {
    // Tell Cargo to re-run if these files change
    println!("cargo:rerun-if-changed=c_src/example.c");
    
    // Compile the C code
    cc::Build::new()
        .file("c_src/example.c")
        .compile("example");  // Produces libexample.a
}