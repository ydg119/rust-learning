mod thread {
    pub mod producer_consumer;
    pub mod thread_scope;
}
fn main() {
    // thread scope process
    println!("thread scope process");
    thread::thread_scope::process();

    // producer consumer process
    println!("producer consumer process");
    thread::producer_consumer::process();
}
