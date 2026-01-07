mod producer_consumer;
mod thread_scope;
fn main() {
    // thread scope process
    println!("thread scope process");
    thread_scope::process();

    // producer consumer process
    println!("producer consumer process");
    producer_consumer::process();
}
