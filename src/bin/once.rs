use std::sync::Once;

static INIT: Once = Once::new();
static mut DATA: Option<u32> = None;

fn main() {
    let handle1 = std::thread::spawn(|| {
        INIT.call_once(|| {
            unsafe { DATA = Some(42) };
            println!("Initialized DATA in thread 1");
        });
    });

    let handle2 = std::thread::spawn(|| {
        INIT.call_once(|| {
            unsafe { DATA = Some(42) };
            println!("Initialized DATA in thread 2");
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final DATA: {:?}", DATA);
    }
}
