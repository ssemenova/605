/*
Simple example of ThreadGroups and intra-group communication with a syntactic
alternative to mpsc channels. Rather than creating an mpsc channel between
two threads, each thread is wrapped with a ThreadGroup object, then linked
to another thread. Each link creates a channel under-the-hood.
*/
extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup, TaggedThread};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;


struct GroupA;
impl GroupTag for GroupA { }

struct GroupB;
impl GroupTag for GroupB { }


fn main() {
    // Create two groups
    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();


    /* GROUP A */
    // Create two TaggedThreads, where the parameter is the thread function.
    let group_a_t1 = TaggedThread::new(produce);
    let group_a_t2 = TaggedThread::new(consume);

    // Link t1 and t2. This creates an mpsc channel from t1->t2
    let (group_a_t1, group_a_t2) = group_a.link(group_a_t1, group_a_t2);
    
    // Spawn the two threads. This time, all that needs to be passed in
    // is the TaggedThread object.
    group_a.spawn_thread(group_a_t1);
    group_a.spawn_thread(group_a_t2);


    /* GROUP B */
    // Create two TaggedThreads, where the parameter is the thread function.
    let group_b_t1 = TaggedThread::new(produce);
    let group_b_t2 = TaggedThread::new(consume);

    // Link t1 and t2. This creates an mpsc channel from t1->t2
    let (group_b_t1, group_b_t2) = group_b.link(group_b_t1, group_b_t2);
    
    // Spawn the two threads. This time, all that needs to be passed in
    // is the TaggedThread object.
    group_b.spawn_thread(group_b_t1);
    group_b.spawn_thread(group_b_t2);


    // Join threads
    group_a.wait();
    group_b.wait();
}


fn produce(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let _ = r;

    let mut i = 0;
    loop {
        let _ = tx.send(i);
        i += 1;
        sleep(Duration::from_secs(1));
    }
}


fn consume(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = s;
    let rx = r.get(0).unwrap();

    loop {
        let i = rx.recv().unwrap();
        println!("Consuming {}", i);
    }
}
