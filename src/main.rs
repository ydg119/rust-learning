mod basic;
mod data_structures;
mod thread;

use data_structures::queue::Queue;

fn main() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(9);
    println!("queue: {:?}", queue);
    println!("queue is empty? : {}", queue.is_empty());
    queue.dequeue();
    println!("queue: {:?}", queue);
    println!("queue is empty? : {}", queue.is_empty());
    queue.dequeue();
    println!("queue: {:?}", queue);
    println!("queue is empty? : {}", queue.is_empty());
}
