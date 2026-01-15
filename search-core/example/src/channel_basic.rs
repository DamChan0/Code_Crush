use std::sync::mpsc;
use std::thread;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello from the thread");
    });

    let msg = rx.recv().unwrap();
    println!("Message received: {:?}", msg);
}
