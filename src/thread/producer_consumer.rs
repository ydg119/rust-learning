use std::sync::mpsc;
use std::thread;
pub fn process() {
    let (tx, rx) = mpsc::channel();
    // producer
    let producer = thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        for x in data {
            tx.send(x).unwrap();
            println!("Send data: {}", x);
        }
    });

    let comsumer = thread::spawn(move || {
        for received in rx {
            println!("Received data: {}", received);
        }
    });

    producer.join().unwrap();
    comsumer.join().unwrap();
}
