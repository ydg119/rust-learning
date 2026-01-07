use std::sync::mpsc;
use std::thread;

pub fn process() {
    // Use a scope to avoid having to join all the threads
    thread::scope(|scope| {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        // Stage 1
        scope.spawn(move || {
            let data = vec![1, 2, 3, 4, 5];
            for x in data {
                // Sauqres each number
                tx1.send(x * x).unwrap();
            }
        });

        // Stage 2
        scope.spawn(move || {
            for x in rx1 {
                // Increments each number
                tx2.send(x + 1).unwrap();
            }
        });

        scope.spawn(move || {
            for y in rx2 {
                // print result
                println!("pipeline output: {}", y);
            }
        });
    });
}
