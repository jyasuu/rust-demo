use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 1. Define the custom Future
struct MyManualFuture {
    message: String,
}

impl Future for MyManualFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // In this simple case, we return Ready immediately
        Poll::Ready(self.message.clone())
    }
}

#[tokio::main]
async fn main() {
    // 2. Using your manual Future
    let manual_future = MyManualFuture {
        message: "Hello from the manual Future!".to_string(),
    };

    // We .await it to trigger the 'poll' method
    let result_manual = manual_future.await;
    println!("Manual Future Result: {}", result_manual);

    // 3. Using your async block
    let message = "Hello from the async block!".to_string();
    let my_async_block = async move {
        message
    };

    let result_block = my_async_block.await;
    println!("Async Block Result: {}", result_block);
}






// We need a "do-nothing" waker to satisfy the poll requirement
fn dummy_waker() -> Waker {
    fn raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { raw_waker() }
        let vtable = &RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(std::ptr::null(), vtable)
    }
    unsafe { Waker::from_raw(raw_waker()) }
}

fn main() {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    
    // 2. Using your manual Future
    let manual_future = MyManualFuture {
        message: "Hello from the manual Future!".to_string(),
    };


    // 3. Using your async block
    let message = "Hello from the async block!".to_string();
    let my_async_block = async move {
        message
    };
    
    
    // We must "Pin" the future before polling it
    let mut pinned_future = std::pin::pin!(manual_future);

    // Manually trigger one poll
    if let Poll::Ready(val) = pinned_future.as_mut().poll(&mut cx) {
        println!("Manual Future Result {}", val);
    }
    // We must "Pin" the future before polling it
    let mut pinned_future = std::pin::pin!(my_async_block);

    // Manually trigger one poll
    if let Poll::Ready(val) = pinned_future.as_mut().poll(&mut cx) {
        println!("Async Block Result {}", val);
    }
}
