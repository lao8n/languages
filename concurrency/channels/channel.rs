use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){
    let (tx, rx) = mpsc::channel();

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