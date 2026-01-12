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
