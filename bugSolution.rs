use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));

    let v1 = v.clone();
    let handle1 = thread::spawn(move || {
        let mut data = v1.lock().unwrap();
        *data = vec![10,20,30];
    });

    let v2 = v.clone();
    let handle2 = thread::spawn(move || {
        let mut data = v2.lock().unwrap();
        data.push(40);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result: {:?}", *v.lock().unwrap());
} 