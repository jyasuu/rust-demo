use std::{sync::{Arc, Mutex, Once}, thread};

pub struct Singleton {
    data: String,
}

static INIT: Once = Once::new();
static mut INSTANCE: Option<Mutex<Singleton>> = None;

impl Singleton {
    pub fn get_instance() -> &'static Mutex<Singleton> {
        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(Mutex::new(Singleton {
                    data: "Initial".to_string(),
                }));
            });
            INSTANCE.as_ref().unwrap()
        }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

// Usage
// Data: New Data
// Data in thread: New Data
// Data in thread: New Data 0
// Data in thread: New Data 2
// Data in thread: New Data 1
// Data in thread: New Data 3
// Data in thread: New Data 4
// Data in thread: New Data 5
// Data in thread: New Data 6
// Data in thread: New Data 7
// Data in thread: New Data 8
// Data: New Data 9
fn main() {
    {
        let instance = Singleton::get_instance();
        {
            let mut guard = instance.lock().unwrap();
            guard.set_data("New Data".to_string());
            println!("Data: {}", guard.get_data());
        }
        
        let instance = Arc::new(instance);
        
        let mut handles = vec![];
        for i in 0..10 {
            let instance = Arc::clone(&instance);
            let handle = thread::spawn(move || {
                let mut guard = instance.lock().unwrap();
                println!("Data in thread: {}", guard.get_data());
                guard.set_data(format!("New Data {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
    
    {
        let instance = Singleton::get_instance();
        {
            let guard = instance.lock().unwrap();
            println!("Data: {}", guard.get_data());
        }
        
    }
}