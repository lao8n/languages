use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// TODO: implemented as a ring buffer (bounded channels)
fn main(){
    // transmitter and receiver
    // unlike golang rust defaults to buffered channels
    // uses 1:1 threads where 1 language thread to 1 os thread.
    let (tx, rx) = mpsc::channel(); // clear ownership of sender and receivers makes it easier to automatically detect when a channel should be closed

    // ownership in 1. thread 2. channel 3. in receiver
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // tx.send takes ownership
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}