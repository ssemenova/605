use std::thread;
use std::sync::mpsc::channel;

mod GroupedThread;
mod GroupedThreadChannel;

fn main() {
    normalThreadExample();
    groupedThreadExample();
}


fn normalThreadExample() {
    // Example of letting two threads talk to each other
    // using mpsc and normal threads

    let (tx2to1, rx1) = channel();
    let (tx1to2, rx2) = channel();

    let thread1_handle = thread::spawn(move || {
        tx1to2.send(0).unwrap();
        let received = rx1.recv().unwrap();
        println!("Thread 1 received {} from thread 2 channel", received);
    });

    let thread2_handle = thread::spawn(move || {
        tx2to1.send(1).unwrap();
        let received = rx2.recv().unwrap();
        println!("Thread 2 received {} from thread 1 channel", received);
    });

    thread1_handle.join().unwrap();
    thread2_handle.join().unwrap();
}

fn groupedThreadExample() {
    
} 