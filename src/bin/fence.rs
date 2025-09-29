use std::sync::atomic::{AtomicBool, fence, Ordering};
use std::thread;

static mut DATA: u32 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    // 寫入線程
    thread::spawn(|| {
        unsafe { DATA = 42 };        // 1. 寫入數據
        
        fence(Ordering::Release);    // 2. 建立記憶體屏障
        
        READY.store(true, Ordering::Relaxed); // 3. 設置準備標誌
    });

    // 讀取線程  
    thread::spawn(|| {
        while !READY.load(Ordering::Relaxed) {
            // 等待準備完成
            println!("Data is: {}", unsafe { DATA });
        }
        
        fence(Ordering::Acquire);    // 對應的獲取屏障
        
        println!("Data is: {}", unsafe { DATA }); // 保證看到 42
    });

    thread::sleep(std::time::Duration::from_secs(1));
}
