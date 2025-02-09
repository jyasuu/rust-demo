use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::task;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EventBus {
    subscribers: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<Event>>>>>,
}

#[derive(Debug, Clone)]
pub struct Event(String); // A simple event type for demonstration

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn publish(&self, topic: &str, event: Event) {
        if let Some(subscribers) = self.subscribers.read().unwrap().get(topic) {
            for subscriber in subscribers.iter() {
                let _ = subscriber.send(event.clone()).await;
            }
        }
    }

    pub fn subscribe(&self, topic: &str) -> (mpsc::Receiver<Event>, impl FnOnce()) {
        let (tx, rx) = mpsc::channel(10); // Buffered channel of size 10
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx.clone());

        let topic = topic.to_string();
        let subscribers = Arc::clone(&self.subscribers);

        let unsubscribe = move || {
            let mut subscribers = subscribers.write().unwrap();
            if let Some(channels) = subscribers.get_mut(&topic) {
                channels.retain(|ch| !Arc::ptr_eq(&Arc::new(ch.clone()), &Arc::new(tx.clone())));
            }
        };

        (rx, unsubscribe)
    }
}

// Custom error for notification handling
#[derive(Debug)]
struct NotificationError(String);

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Notification Error: {}", self.0)
    }
}

impl Error for NotificationError {}

struct NotificationService {
    event_bus: Arc<EventBus>,
    handlers: HashMap<String, Arc<dyn Fn(Event) -> Result<(), Box<dyn Error>> + Send + Sync>>,
}

impl NotificationService {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        let mut service = Self {
            event_bus,
            handlers: HashMap::new(),
        };
        service.register_handlers();
        service.subscribe_to_events();
        service
    }

    fn register_handlers(&mut self) {
        self.handlers.insert("base.members.viewed".to_string(), Arc::new(|event| {
            println!("Handling base.members.viewed event: {:?}", event);
            Ok(())
        }));
        self.handlers.insert("base.created".to_string(), Arc::new(|event| {
            println!("Handling base.created event: {:?}", event);
            Ok(())
        }));
    }

    fn subscribe_to_events(&self) {
        for (event_type, handler) in self.handlers.iter().map(|(k, v)| (k.clone(), Arc::clone(v))) {
            let (mut events, unsubscribe) = self.event_bus.subscribe(&event_type);
            task::spawn(async move {
                let _unsubscribe = unsubscribe;
                println!("Listening for {} events", event_type);
                while let Some(event) = events.recv().await {
                    if let Err(e) = handler(event) {
                        println!("Failed to handle {} event: {}", event_type, e);
                    }
                }
            });
        }
    }
}

#[tokio::main]
async fn main() {
    let event_bus = Arc::new(EventBus::new());
    let _notification_service = NotificationService::new(Arc::clone(&event_bus));

    event_bus
        .publish("base.members.viewed", Event("Sample Base Viewed Event".into()))
        .await;
    event_bus
        .publish("base.created", Event("Sample Base Created Event".into()))
        .await;
}
