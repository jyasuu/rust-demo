use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }


    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);

    
    let mut num = 42;
    
    // Create raw pointers (safe operation)
    let ptr_imm: *const i32 = &num as *const i32;
    let ptr_mut: *mut i32 = &mut num as *mut i32;

    unsafe {
        // Dereference raw pointers (requires unsafe)
        println!("Mutated: {}", *ptr_mut);
        println!("Immutable: {}", *ptr_imm);
        *ptr_mut = 100;
        println!("Mutated: {}", *ptr_mut);
        println!("Immutable: {}", *ptr_imm);
    }

    
    let mut data = 10;
    let mut ptr: *mut i32 = &mut data;
    
    // Valid pointer access
    unsafe {
        *ptr *= 2;
    }
    println!("Doubled: {}", data);

    // Set to null pointer
    ptr = std::ptr::null_mut();
    
    unsafe {
        // Safe null check before dereference
        if !ptr.is_null() {
            *ptr = 100; // Would crash without check
        } else {
            println!("Pointer is null!");
        }
    }
    println!("Doubled: {}", data);


    
    let arr = [10, 20, 30, 40];
    let ptr: *const i32 = arr.as_ptr();

    unsafe {
        // Access array elements through pointer arithmetic
        for i in 0..arr.len() {
            let elem: i32 = *ptr.add(i); // Equivalent to ptr.offset(i as isize)
            println!("Element {}: {}", i, elem);
        }
    }
}