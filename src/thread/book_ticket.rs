use std::sync::Arc;
use std::thread;

struct Theater {
    available_tickets: *mut i32,
}

unsafe impl Send for Theater {}
unsafe impl Sync for Theater {}

impl Theater {
    fn new(initial_tickets: i32) -> Self {
        Theater {
            available_tickets: Box::into_raw(Box::new(initial_tickets)),
        }
    }

    fn book_ticket(&self) {
        unsafe {
            if *self.available_tickets > 0 {
                thread::sleep(std::time::Duration::from_millis(10));
                *self.available_tickets -= 1;
                println!(
                    "Ticket booked! Remaining tickets: {}",
                    *self.available_tickets
                );
            } else {
                println!("Sorry, no more tickets available.");
            }
        }
    }

    fn get_available_tickets(&self) -> i32 {
        unsafe { *self.available_tickets }
    }
}

impl Drop for Theater {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.available_tickets)) }
    }
}

pub fn test() {
    let theater = Arc::new(Theater::new(10));
    let mut handles = vec![];
    for _ in 0..15 {
        let theater_clone = Arc::clone(&theater);
        let handle = thread::spawn(move || {
            theater_clone.book_ticket();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final ticket count: {}", theater.get_available_tickets());
}
