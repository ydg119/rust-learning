use std::sync::mpsc;
use std::thread;

pub fn test() {
    let (tx, rx) = mpsc::channel();
    // fan-in: distributing work to multiple threads
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // each worker sends a message
            tx_clone.send(i * 2).unwrap();
        });
    }
    // close the original sender
    drop(tx);

    // fan-out: collecting results from multiple threads
    for received in rx {
        println!("fan-out received: {}", received);
    }
}
