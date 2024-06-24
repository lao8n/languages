use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| { // return type of thread::spawn is JoinHandle
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1)); // makes thread non-runnable and doesn't consume cpu 
    }

    handle.join().unwrap(); // join blocks main thread waiting for other thread to return
}