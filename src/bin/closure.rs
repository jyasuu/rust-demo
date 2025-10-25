// ============================================
// RUST FUNCTION TRAITS: Fn, FnMut, FnOnce
// ============================================

// Quick Reference:
// - Fn: Can be called multiple times, doesn't modify captured variables
// - FnMut: Can be called multiple times, CAN modify captured variables
// - FnOnce: Can be called only ONCE, takes ownership of captured variables

fn main() {
    // ========== EXAMPLE 1: Fn TRAIT ==========
    println!("=== Fn Trait ===");
    
    let x = 5;
    let add_x = |a| a + x;  // Captures x by reference (immutable)
    
    println!("First call: {}", add_x(10));   // 15
    println!("Second call: {}", add_x(20));  // 25
    // Can call multiple times because it only borrows x immutably
    
    fn call_twice_fn<F: Fn(i32) -> i32>(f: F, a: i32, b: i32) -> i32 {
        f(a) + f(b)
    }
    
    let result = call_twice_fn(add_x, 3, 4);
    println!("call_twice_fn result: {}\n", result);  // (3+5) + (4+5) = 17


    // ========== EXAMPLE 2: FnMut TRAIT ==========
    println!("=== FnMut Trait ===");
    
    let mut counter = 0;
    let mut increment = || {
        counter += 1;  // Modifies captured variable
        counter
    };
    
    println!("First call: {}", increment());   // 1
    println!("Second call: {}", increment());  // 2
    println!("Third call: {}", increment());   // 3
    // Can call multiple times, and counter gets modified each time
    
    fn call_twice_fnmut<F: FnMut() -> i32>(mut f: F) -> i32 {
        f() + f()
    }
    
    let mut count2 = 0;
    let mut inc2 = || {
        count2 += 1;
        count2
    };
    let result = call_twice_fnmut(inc2);
    println!("call_twice_fnmut result: {}\n", result);  // (1) + (2) = 3


    // ========== EXAMPLE 3: FnOnce TRAIT ==========
    println!("=== FnOnce Trait ===");
    
    let s = String::from("hello");
    let consume = || {
        println!("Consuming: {}", s);  // Takes ownership of s
    };
    
    consume();  // First call
    // consume();  // ERROR! Can't call again - s was moved
    
    fn call_once_fnonce<F: FnOnce()>(f: F) {
        f();  // FnOnce consumes the closure
    }
    
    let s2 = String::from("world");
    let consume2 = || println!("Consuming: {}", s2);
    call_once_fnonce(consume2);
    // s2 is gone, consume2 is gone
    println!();


    // ========== EXAMPLE 4: COMPARISON ==========
    println!("=== Comparison Example ===");
    
    let vec = vec![1, 2, 3];
    
    // Fn: only reads
    let print_fn = || println!("Fn reads: {:?}", vec);
    print_fn();
    print_fn();
    println!("vec still exists: {:?}", vec);
    
    // FnMut: modifies but doesn't take ownership
    let mut vec2 = vec![1, 2, 3];
    let mut modify_fnmut = || vec2.push(4);
    modify_fnmut();
    modify_fnmut();
    println!("FnMut modified vec: {:?}", vec2);
    
    // FnOnce: takes ownership
    let vec3 = vec![1, 2, 3];
    let consume_once = || {
        println!("FnOnce takes ownership: {:?}", vec3);
        // vec3 moved here, can't use after this
    };
    consume_once();
    // println!("{:?}", vec3);  // ERROR! vec3 was consumed
    println!();


    // ========== PRACTICAL EXAMPLE: CALLBACKS ==========
    println!("=== Practical: Processing with Callbacks ===");
    
    fn process_with_callback<F: Fn(&str)>(data: &str, callback: F) {
        println!("Processing: {}", data);
        callback(data);
    }
    
    let prefix = "Result: ";
    process_with_callback("Hello", |s| println!("{}{}", prefix, s));
    process_with_callback("World", |s| println!("{}{}", prefix, s));
}

// ========== WHEN TO USE EACH ==========
/*
Use Fn when:
  - The closure needs to be called multiple times
  - It only reads captured variables
  - Example: map, filter, closures that transform data

Use FnMut when:
  - The closure needs to be called multiple times
  - It needs to modify captured variables
  - Example: sort_by, closures that accumulate state

Use FnOnce when:
  - The closure is called exactly once
  - It takes ownership of captured variables
  - It might consume the variable
  - Example: into_iter, once()
*/
