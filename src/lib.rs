// Include generated code from build directory
include!(concat!(env!("OUT_DIR"), "/generated_config.rs"));

// Use the generated functions
pub fn use_generated() {
    println!("{}", get_data());
    println!("{}", send_data());
}