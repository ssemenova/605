/*
Example of breaking our code with global mutexes.
Global mutex is created inside a lazy_static macro and can be accessed
between two threads in different thread groups.
*/
extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
use std::sync::Mutex;

use lazy_static::lazy_static;


struct GroupA;
impl GroupTag for GroupA { }

struct GroupB;
impl GroupTag for GroupB { }


static GLOBAL_INT: i32 = 0;
lazy_static! {
    static ref GLOBAL_MUT: Mutex<i32> = Mutex::new(GLOBAL_INT);
}


fn main() {
    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();

    group_a.spawn(counter1, vec![], vec![]);

    group_b.spawn(counter2, vec![], vec![]);

    group_a.wait();
    group_b.wait();
}


fn counter1(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    loop {
        let mut num = GLOBAL_MUT.lock().unwrap();
        *num += 1; 
        println!("Thread 1 '{}'", num);
        sleep(Duration::from_secs(1));
    }
}


fn counter2(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    loop {
        let mut num = GLOBAL_MUT.lock().unwrap();
        *num += 1; 
        println!("Thread 2 '{}'", num);
        sleep(Duration::from_secs(1));
    }
}
