fn main() {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::thread;
    println!("start");
    let flag = Arc::new(AtomicBool::new(false));
    let child_thread = thread::spawn({
        println!("start thread");
        let flag = Arc::clone(&flag);
        move || {
            println!("start once");
            while !flag.load(Ordering::Relaxed) {
                println!("thread park");
                thread::park(); // 暂停，直到被唤醒
                println!("thread unparked");
            }
            println!("条件满足，子线程退出");
        }
    });
    println!("main sleep");
    // 主线程设置条件并唤醒子线程
    thread::sleep(std::time::Duration::from_secs(2));
    println!("main wake");
    flag.store(true, Ordering::Relaxed);
    println!("main unpark");
    child_thread.thread().unpark(); // 唤醒子线程
    println!("main done");
}


/* 
start
start thread
main sleep
start once
thread park
...
main wake
main unpark
main done
_thread unparked
_条件满足，子线程退出
*/

