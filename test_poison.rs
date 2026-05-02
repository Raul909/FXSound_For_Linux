use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let m_clone = Arc::clone(&m);

    let _ = thread::spawn(move || {
        let _lock = m_clone.lock().unwrap();
        panic!("poisoning mutex");
    }).join();

    match m.lock() {
        Ok(_) => println!("Lock OK"),
        Err(_) => println!("Lock Poisoned"),
    }

    let val = m.lock().unwrap_or_else(|e| e.into_inner());
    println!("Recovered value: {}", *val);
}
