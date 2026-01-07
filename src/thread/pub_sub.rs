use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;

type SubscriberId = usize;
struct PubSub<T> {
    subscribers: Arc<Mutex<BTreeMap<SubscriberId, SyncSender<T>>>>,
    next_id: AtomicUsize,
}

impl<T: Send + 'static> PubSub<T> {
    fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: <_>::default(),
        }
    }
    fn subscribe(&self) -> (SubscriberId, Receiver<T>) {
        let (tx, rx) = mpsc::sync_channel(512);
        let mut subscribers = self.subscribers.lock().unwrap();
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        subscribers.insert(id, tx);
        (id, rx)
    }

    fn unsubscribe(&self, id: SubscriberId) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.remove(&id);
    }

    fn publish_deadlocking(&self, message: T)
    where
        T: Clone,
    {
        let subscribers = self.subscribers.lock().unwrap();
        for tx in subscribers.values() {
            tx.send(message.clone()).unwrap();
        }
    }

    fn publish(&self, message: T)
    where
        T: Clone,
    {
        let subscribers = self.subscribers.lock().unwrap();
        for tx in subscribers.values() {
            let _ = tx.send(message.clone());
        }
    }
}

pub fn test() {
    let hub = PubSub::new();
    let (sub_id, rx) = hub.subscribe();
    let handle = thread::spawn(move || {
        for received in rx {
            println!("Received: {:?}", received);
        }
    });
    hub.publish("Hello, world!");
    hub.unsubscribe(sub_id);
    handle.join().unwrap();
}
