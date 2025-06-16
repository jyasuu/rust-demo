trait Observer {
    fn update(&self, event: &str);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, event: &str) {
        for observer in &self.observers {
            observer.update(event);
        }
    }
}

struct EmailNotifier {
    email: String,
}

impl Observer for EmailNotifier {
    fn update(&self, event: &str) {
        println!("Email to {}: Event '{}' occurred", self.email, event);
    }
}

struct SMSNotifier {
    phone: String,
}

impl Observer for SMSNotifier {
    fn update(&self, event: &str) {
        println!("SMS to {}: Event '{}' occurred", self.phone, event);
    }
}

// Usage
fn main() {
    let mut subject = Subject::new();
    
    subject.attach(Box::new(EmailNotifier {
        email: "user@example.com".to_string(),
    }));
    
    subject.attach(Box::new(SMSNotifier {
        phone: "+1234567890".to_string(),
    }));

    subject.notify("System Startup");
    subject.notify("Data Updated");
}