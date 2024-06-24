use std::sync::mpsc; // multiple producer single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); 

    spawn_sender(tx.clone(), vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ]);
    spawn_sender(tx, vec![ // have to have tx last as clone borrows after move o/w
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ]);

    for received in rx {
        println!("Got: {received}");
    }
    // unlike golang do not need to close a channel because when all references are dropped to a tx when no senders left
    // note the converse is not true so if receivers destructed the senders will not be - they will error instead
}

fn spawn_sender(tx: mpsc::Sender<String>, messages: Vec<String>) {
    thread::spawn(move || {
        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}