use std::sync::{Arc, Mutex};

// Define trait Observer
pub trait Observer {
    fn update(&self, message: &str);
}

// Implementasi Observer untuk LoggerObserver
pub struct LoggerObserver;

impl Observer for LoggerObserver {
    fn update(&self, message: &str) {
        println!("Logging: {}", message);
    }
}

// Struktur Observable yang diperbarui untuk CRUD
pub struct Observable {
    observers: Mutex<Vec<Arc<dyn Observer + Send + Sync>>>,
}

impl Observable {
    pub fn new() -> Self {
        Observable {
            observers: Mutex::new(Vec::new()),
        }
    }

    pub fn add_observer(&self, observer: Arc<dyn Observer + Send + Sync>) {
        self.observers.lock().unwrap().push(observer);
    }

    // Method untuk mengirim notifikasi sesuai tindakan CRUD
    pub fn notify_crud(&self, action: &str, entity: &str) {
        let message = format!("{} action performed on {}", action, entity);
        self.notify_observers(&message);
    }

    // Method untuk mengirim notifikasi umum
    pub fn notify_observers(&self, message: &str) {
        for observer in self.observers.lock().unwrap().iter() {
            observer.update(message);
        }
    }
}