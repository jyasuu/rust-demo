// ============================================================================
// ASYNC FN + GENERIC TYPES + BOX & DYN PRACTICE GUIDE
// ZERO DEPENDENCIES - NO async_trait, NO tokio
// ============================================================================

use std::fmt::Debug;
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};

// ============================================================================
// LEVEL 1: Basic Box<dyn Trait> without async_trait
// ============================================================================

// Instead of async fn in trait, we return Pin<Box<dyn Future>>
trait Animal: Send {
    fn speak(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async { "Woof!".to_string() })
    }
}

impl Animal for Cat {
    fn speak(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async { "Meow!".to_string() })
    }
}

// Use different types implementing the same trait
async fn demo_box_dyn() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        println!("{}", animal.speak().await);
    }
}

// ============================================================================
// LEVEL 2: Returning Box<dyn Future>
// ============================================================================

// Return a boxed future that can be different concrete types
fn create_async_operation<T: Debug + Send + 'static>(value: T) -> Box<dyn Future<Output = String> + Send> {
    Box::new(async move {
        format!("Value: {:?}", value)
    })
}

// Usage:
// let fut1 = create_async_operation(42);
// let fut2 = create_async_operation("hello");

// ============================================================================
// LEVEL 3: Manual Async Trait with dyn
// ============================================================================

trait DataFetcher: Send + Sync {
    fn fetch(&self, id: u32) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>>;
}

struct ApiClient;
struct DatabaseClient;

impl DataFetcher for ApiClient {
    fn fetch(&self, id: u32) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
        Box::pin(async move {
            // Simulate async I/O
            std::thread::sleep(std::time::Duration::from_millis(10));
            Ok(format!("API data for {}", id))
        })
    }
}

impl DataFetcher for DatabaseClient {
    fn fetch(&self, id: u32) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
        Box::pin(async move {
            // Simulate async I/O
            std::thread::sleep(std::time::Duration::from_millis(5));
            Ok(format!("DB data for {}", id))
        })
    }
}

// Accept any type that implements DataFetcher as a trait object
async fn fetch_from_source(fetcher: &dyn DataFetcher, id: u32) -> Result<String, String> {
    fetcher.fetch(id).await
}

// ============================================================================
// LEVEL 4: Box<dyn Trait> in Collections
// ============================================================================

trait Task: Send + Sync {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>>;
}

struct PrintTask(String);
struct ComputeTask(u32);

impl Task for PrintTask {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> {
        let msg = self.0.clone();
        Box::pin(async move {
            println!("{}", msg);
            Ok(())
        })
    }
}

impl Task for ComputeTask {
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> {
        let value = self.0;
        Box::pin(async move {
            std::thread::sleep(std::time::Duration::from_millis(value as u64));
            println!("Computed: {}", value * 2);
            Ok(())
        })
    }
}

async fn run_tasks(tasks: Vec<Box<dyn Task>>) {
    for task in tasks {
        if let Err(e) = task.execute().await {
            eprintln!("Task failed: {}", e);
        }
    }
}

// ============================================================================
// LEVEL 5: Type Alias for Common Future Patterns
// ============================================================================

// Simplify complex future type signatures with type aliases
type AsyncFn<T> = Box<dyn Future<Output = T> + Send>;
type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T, String>> + Send>>;

async fn process_generic<T: Send + 'static>(value: T) -> T {
    std::thread::sleep(std::time::Duration::from_millis(1));
    value
}

fn create_pinned_future<T: Send + 'static>(value: T) -> Pin<Box<dyn Future<Output = T> + Send>> {
    Box::pin(async move {
        process_generic(value).await
    })
}

// ============================================================================
// LEVEL 6: Generic Function with dyn Trait Parameter
// ============================================================================

// Generic methods make traits not object-safe, so we need a different approach
trait Serializer: Send + Sync {
    fn serialize(&self, value: String) -> AsyncResult<String>;
}

struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn serialize(&self, value: String) -> AsyncResult<String> {
        Box::pin(async move {
            Ok(format!("JSON: {}", value))
        })
    }
}

async fn save_value(
    serializer: &dyn Serializer,
    value: String,
) -> Result<String, String> {
    serializer.serialize(value).await
}

// ============================================================================
// LEVEL 7: Generic with Box<dyn> Return Type
// ============================================================================

trait Handler<T: Send>: Send + Sync {
    fn handle(&self, value: T) -> AsyncResult<String>;
}

async fn execute_handler<T: Send + 'static>(
    handler: Box<dyn Handler<T>>,
    value: T,
) -> Result<String, String> {
    handler.handle(value).await
}

// ============================================================================
// LEVEL 8: Generic Enum with Box<dyn>
// ============================================================================

enum Operation<T: Send + 'static> {
    Execute(Pin<Box<dyn Future<Output = T> + Send>>),
}

impl<T: Send + 'static> Operation<T> {
    async fn run(self) -> T {
        match self {
            Operation::Execute(fut) => fut.await,
        }
    }
}

// ============================================================================
// LEVEL 9: Trait Object Factory Pattern
// ============================================================================

trait Service: Send + Sync {
    fn process(&self, input: String) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

struct JsonService;
struct XmlService;

impl Service for JsonService {
    fn process(&self, input: String) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async move {
            format!("JSON: {}", input)
        })
    }
}

impl Service for XmlService {
    fn process(&self, input: String) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async move {
            format!("XML: {}", input)
        })
    }
}

fn create_service(service_type: &str) -> Box<dyn Service> {
    match service_type {
        "json" => Box::new(JsonService),
        "xml" => Box::new(XmlService),
        _ => Box::new(JsonService),
    }
}

async fn use_service(input: String) {
    let service = create_service("json");
    let result = service.process(input).await;
    println!("{}", result);
}

// ============================================================================
// LEVEL 10: Generic Struct with Box<dyn Future> Field
// ============================================================================

struct Pipeline<T: Send + 'static> {
    steps: Vec<Box<dyn Future<Output = T> + Send>>,
}

impl<T: Send + 'static> Pipeline<T> {
    fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    fn add_step(&mut self, step: Box<dyn Future<Output = T> + Send>) {
        self.steps.push(step);
    }
}

// ============================================================================
// LEVEL 11: Implementing Custom Future
// ============================================================================

struct DelayedValue<T> {
    value: Option<T>,
    delay: u64,
    started: bool,
}

impl<T> DelayedValue<T> {
    fn new(value: T, delay_ms: u64) -> Self {
        DelayedValue {
            value: Some(value),
            delay: delay_ms,
            started: false,
        }
    }
}

impl<T: Unpin> Future for DelayedValue<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.started {
            self.started = true;
            std::thread::sleep(std::time::Duration::from_millis(self.delay));
        }
        Poll::Ready(self.value.take().expect("Future polled twice"))
    }
}

// ============================================================================
// PRACTICE EXERCISES
// ============================================================================

/*
EXERCISE 1: Box<dyn Trait> with Manual Async
- Create a trait Logger with method fn log(&self) -> AsyncResult<()>
- Implement for FileLogger and ConsoleLogger
- Create a Vec<Box<dyn Logger>> and call each

EXERCISE 2: Generic Function + dyn Trait Object
- Create a generic function that accepts &dyn Handler<T>
- Use it to call different handler implementations

EXERCISE 3: Box<dyn Future> Return Type
- Create a factory function that returns Box<dyn Future<Output = T>>
- Return different concrete futures based on a parameter

EXERCISE 4: Trait Object with Generic Methods
- Create a trait with generic methods returning AsyncResult
- Implement for multiple types
- Use Box<dyn Trait> to store them

EXERCISE 5: Generic Struct with Box<dyn Future> Fields
- Create a struct Worker<T> with a field Vec<Box<dyn Future<Output = T>>>
- Implement async method that polls all futures

EXERCISE 6: Nested Generics + dyn
- Create a generic function that takes Vec<Box<dyn Handler<T>>>
- Process all items asynchronously

EXERCISE 7: Custom Future Implementation
- Implement the Future trait manually for a custom type
- Use it with async/await

EXERCISE 8: Generic with Lifetime Bounds
- Create Box<dyn Trait + '_> with explicit lifetime
- Understand why lifetimes matter with trait objects

EXERCISE 9: Combining Everything
- Create a processing pipeline with:
  - Generic type parameter
  - dyn Trait objects
  - Box<dyn Future> handling
  - Error propagation
*/

// ============================================================================
// SOLUTION SKETCHES
// ============================================================================

// Exercise 1 Solution Sketch:
type AsyncVoid = Pin<Box<dyn Future<Output = ()> + Send>>;

trait Logger: Send + Sync {
    fn log(&self, msg: &str) -> AsyncVoid;
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) -> AsyncVoid {
        let msg = msg.to_string();
        Box::pin(async move {
            println!("LOG: {}", msg);
        })
    }
}

async fn exercise1_demo() {
    let loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(ConsoleLogger),
    ];

    for logger in loggers {
        logger.log("Hello").await;
    }
}

// Exercise 3 Solution Sketch:
fn make_future<T: Send + 'static>(value: T, use_delay: bool) 
    -> Box<dyn Future<Output = T> + Send> 
{
    if use_delay {
        Box::new(async move {
            std::thread::sleep(std::time::Duration::from_millis(100));
            value
        })
    } else {
        Box::new(async move { value })
    }
}

// Exercise 7 Solution Sketch:
struct CountUp {
    current: u32,
    max: u32,
}

impl Future for CountUp {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.current += 1;
        if self.current >= self.max {
            Poll::Ready(self.current)
        } else {
            Poll::Pending
        }
    }
}

// ============================================================================
// KEY PATTERNS TO REMEMBER
// ============================================================================

/*
1. Basic trait with async-like behavior:
   trait MyTrait {
       fn async_method(&self) -> Pin<Box<dyn Future<Output = T> + Send + '_>>;
   }

2. Implementation:
   impl MyTrait for MyType {
       fn async_method(&self) -> Pin<Box<dyn Future<Output = T> + Send + '_>> {
           Box::pin(async { ... })
       }
   }

3. Type alias for futures:
   type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

4. Box<dyn Trait> in Vec:
   let items: Vec<Box<dyn MyTrait>> = vec![Box::new(A), Box::new(B)];

5. Box<dyn Future> return:
   fn foo() -> Box<dyn Future<Output = T> + Send> { Box::new(async { ... }) }

6. Reference to dyn Trait:
   fn foo(obj: &dyn MyTrait) { }

7. Generic + dyn Combined:
   fn foo<T: Send>(handler: Box<dyn Handler<T>>) -> T { }

8. Factory Pattern:
   fn create() -> Box<dyn Trait> {
       if condition { Box::new(A) } else { Box::new(B) }
   }

9. Trait Object Bounds:
   Box<dyn Trait + Send + Sync + 'static>

10. Lifetime in trait object:
    Box<dyn Trait + '_>  // borrowed from current scope

11. Custom Future impl:
    impl Future for MyType {
        type Output = T;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> { }
    }

12. Avoid requiring lifetime parameter:
    Returning impl Future vs returning Pin<Box<dyn Future>>
*/

// ============================================================================
// WHY NO DEPENDENCIES?
// ============================================================================

/*
Without async_trait:
- No proc macro overhead
- More explicit code (you see Pin<Box<dyn Future>>)
- Full control over trait design
- Smaller dependency tree

Without tokio:
- Uses std::thread for simulation
- Core async/await still works
- Focus on language fundamentals
- Portable to any runtime
*/
