use std::sync::mpsc;
use std::thread;
pub fn process() {
    // let us use a scope to avoid having to join all the threads
    thread::scope(|scope| {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        // stage 1
        scope.spawn(move || {
            let data = vec![1, 2, 3, 4, 5];
            for x in data {
                // squares each number
                tx.send(x * x).unwrap();
            }
        });
        // stage 2
        scope.spawn(move || {
            for x in rx {
                // increments each number
                tx2.send(x + 1).unwrap();
            }
        });
        // final output
        scope.spawn(move || {
            for y in rx2 {
                // print result
                println!("{}", y)
            }
        });
    });
}
