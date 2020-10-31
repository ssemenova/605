/*
Simple example of ThreadGroups and intra-group communication through channels.
*/
extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup, TaggedThread};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;


struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}


struct GroupB;
impl GroupTag for GroupB { 
    fn get_tag() -> u64 { 0x41 }
}


fn main() {
    // Create two groups
    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();


    /* GROUP A */
    // Create mpsc channel between two threads in group A
    let (ch1_a_tx, ch1_a_rx) = group_a.channel::<i32>();
    let (ch2_a_tx, ch2_a_rx) = group_a.channel::<i32>();
    
    // Spawn two threads in group A
    // where the second argument is a list of transmitters
    // and the third argument is a list of receivers
    // created with mpsc
    group_a.spawn(produce, vec![ch1_a_tx], vec![ch2_a_rx]);
    group_a.spawn(consume, vec![ch2_a_tx], vec![ch1_a_rx]);

    
    /* GROUP B */
    // Create mpsc channel between two threads in group B
    let (ch2_a_tx, ch2_a_rx) = group_b.channel::<i32>();
    let (ch3_a_tx, ch3_a_rx) = group_b.channel::<i32>();
    
    // Spawn two threads in group B
    // in the same way as group A
    group_b.spawn(produce, vec![ch2_a_tx], vec![ch3_a_rx]);
    group_b.spawn(consume, vec![ch3_a_tx], vec![ch2_a_rx]);


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
