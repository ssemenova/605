/*
Commented-out examples of code that breaks the library and is checked at compile time.
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
    intergroup_messages_default();
    intergroup_messages_with_linking();
}


// An example of a compiler error when attempting to send messages between two groups,
// using default mpsc channel.
fn intergroup_messages_default() {
    // Create two groups - A and B
    let mut group_a = ThreadGroup::<GroupA>::new();
    let _group_b = ThreadGroup::<GroupB>::new();

    // Create a channel so t1, within group A, can send messages to t2, within group B.
    let (transmitter, _receiver) = group_a.channel::<i32>();

    // Spawn a thread in group A that produces and transmits through the transmitter.
    group_a.spawn(produce, vec![transmitter], vec![]);
    
    /*
        Attempting to spawn t2 within group B with a receiver tied to group A
        will give a compile-time error.
            mismatched types
            expected struct `anodize::IntragroupReceiver<_, GroupB>`
            found struct `anodize::IntragroupReceiver<_, GroupA>`
    */
    // _group_b.spawn(consume, vec![], vec![_receiver]);
}


// An example of a compiler error when attempting to send messages between two groups,
// using the syntactic linking construct instead of explicit mpsc channels.
fn intergroup_messages_with_linking() {
    // Create two groups - A and B
    let mut group_a = ThreadGroup::<GroupA>::new();
    let _group_b = ThreadGroup::<GroupB>::new();

    // Create two TaggedThreads. T1 will be in group A and will produce.
    // T2 will be in group B and consume T1's output.
    let group_a_t1 = TaggedThread::new(produce);
    let group_b_t2 = TaggedThread::new(consume);

    // Link t1 and t2. This creates an mpsc channel from t1->t2
    // This compiles because group_b_t2 and group_a_t1 are not tied to 
    // groups yet.
    let (group_a_t1, _group_b_t2) = group_a.link(group_a_t1, group_b_t2);
    
    // Spawn t1 within group A.
    group_a.spawn_thread(group_a_t1);

    /*
        Attempting to spawn t2 within group B will give a compile-time error.
            mismatched types
            expected struct `anodize::TaggedThread<GroupB>`
            found struct `anodize::TaggedThread<GroupA>`
    */
    // _group_b.spawn_thread(group_b_t2);
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
