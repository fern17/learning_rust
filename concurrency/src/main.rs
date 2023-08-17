use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use num::{BigUint, One};
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
    }
}

fn multithreaded_factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc * x)
    }
}

fn main() {
    // Create a thread
    let handle = std::thread::spawn(move || {
        println!("Hello from a thread!");
    });
    // tell main thread to sleep
    thread::sleep(Duration::from_secs(1)); 

    // waiting until the thread represented by handle is finished
    handle.join().unwrap(); 
    println!("Hello from main");

    let v = vec![1, 2, 3];
    // the move keyword forces the closure to take ownership of the values it uses
    let h = std::thread::spawn(move || { 
        println!("{:?}", v)
    });

    let v2 = vec![1, 2, 3];
    let mut thread_handles = Vec::new();
    for e in v2 {
        thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
    }

    println!("Main thread");
    for handle in thread_handles {
        handle.join().unwrap();
    }

    let (transmitter, receiver) = mpsc::channel(); // multi producer, single consumer
    let tx = transmitter.clone(); // another transmitter that can communicate to the receiver
    let val = String::from("Transmitting!"); // val is defined in the main thread
    std::thread::spawn(move || {
        transmitter.send(val).unwrap(); // take ownership of val and send message to main
        // val is in the second thread now
    });

    let msg = receiver.recv().unwrap(); // wait for message and takes ownership of the value
    // val (now msg) is in the main thread again
    println!("{}", msg);

    let (tx1, rc) = mpsc::sync_channel(1000); 
    // 1000 is the size of the queue (how many messages can handle).
    // When the limit is surpassed, sending messages becomes blocking until the queue is reduced
    let tx2 = tx1.clone(); // another transmitter that can communicate to the receiver

    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"), String::from("From"), String::from("Original")];
        for val in vec {
            tx1.send(val).unwrap();
        }
    });
    std::thread::spawn(move || {
        let vec = vec![String::from("Clone"), String::from("is"), String::from("transmitting")];
        for val in vec {
            tx2.send(val).unwrap();
        }
    });

    for rec in rc {
        println!("{}", rec);
    }

    let counter = Arc::new(Mutex::new(0)); // counter can only be accessed in only one thread
    let mut handles = vec![];
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = c.lock().unwrap(); // c(counter) is locked, cannot be accessed in another thread
            // if we lock counter again, we would have a deadlock
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap()); // 8

    //-----------------------
    println!("Rayon sample");
    
    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());

    let now = Instant::now();
    multithreaded_factorial(50000);
    println!("{:.2?}", now.elapsed());

}
