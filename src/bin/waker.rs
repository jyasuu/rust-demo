use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker, Wake},
    thread,
    time::Duration,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

/// 1. Shared state between the Future and the background thread.
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

/// 2. The custom Future.
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn a thread to simulate an asynchronous event
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = thread_shared_state.lock().unwrap();
            state.completed = true;
            // Wake the task so the executor knows to poll again
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();

        if state.completed {
            Poll::Ready("Timer finished!".to_string())
        } else {
            // BEST PRACTICE: Efficiently update the waker.
            // Check if the waker changed before cloning to avoid atomic overhead.
            match &state.waker {
                Some(existing) if existing.will_wake(cx.waker()) => {}
                _ => {
                    // Use clone_from if the waker exists to reuse the allocation
                    if let Some(w) = state.waker.as_mut() {
                        w.clone_from(cx.waker());
                    } else {
                        state.waker = Some(cx.waker().clone());
                    }
                }
            }
            Poll::Pending
        }
    }
}

/// 3. A minimal Executor to run our future.
struct Task {
    // The future we are running (pinned to the heap)
    future: Mutex<Option<Pin<Box<dyn Future<Output = String> + Send + 'static>>>>,
    // Channel to signal the executor to poll again
    executor_tx: SyncSender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        // Send the task back to the executor queue when woken
        self.executor_tx.send(self.clone()).expect("Executor queue full");
    }
}

fn main() {
    let (tx, rx): (SyncSender<Arc<Task>>, Receiver<Arc<Task>>) = sync_channel(100);

    // Create our timer future
    let timer_future = TimerFuture::new(Duration::from_secs(2));

    // Wrap it in a Task
    let task = Arc::new(Task {
        future: Mutex::new(Some(Box::pin(timer_future))),
        executor_tx: tx.clone(),
    });

    // Initial "kickstart" by sending the task to the executor
    tx.send(task.clone()).unwrap();

    println!("Starting executor...");

    // Executor loop
    while let Ok(task) = rx.recv() {
        let mut future_slot = task.future.lock().unwrap();
        if let Some(mut future) = future_slot.take() {
            // Create a Waker from our Arc<Task>
            let waker = Waker::from(task.clone());
            let mut cx = Context::from_waker(&waker);

            // Poll the future
            if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
                println!("Result: {}", result);
                break; // Future finished, exit executor
            } else {
                println!("Pending");
                // Future is still pending, put it back in the task slot
                *future_slot = Some(future);
            }
        }
    }
}
