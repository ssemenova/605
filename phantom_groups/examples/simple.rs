#[allow(unused_variables)]

mod thread_group;

use std::sync::mpsc::{Sender, Receiver};
use crate::thread_group::{GroupTag, ThreadGroup, TaggedThread};
use std::thread::sleep;
use std::time::Duration;

struct GroupA;
impl GroupTag for GroupA { }

struct GroupB;
impl GroupTag for GroupB { }

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

fn main() {

    // Create groups
    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();

    // Create channels 
    let (ch1_a_tx, ch1_a_rx) = group_a.channel::<i32>();
    let (ch2_a_tx, ch2_a_rx) = group_a.channel::<i32>();

    // Spawn threads
    group_a.spawn(produce, vec![ch1_a_tx], vec![ch2_a_rx]);
    group_a.spawn(consume, vec![ch2_a_tx], vec![ch1_a_rx]);

    let t1 = TaggedThread::new(produce);
    let t2 = TaggedThread::new(consume);

    let (t1, t2) = group_b.link(t1, t2);

    group_b.spawn_thread(t1);
    group_b.spawn_thread(t2);
    
    // group_b.spawn(produce, vec![ch3_a_tx], vec![]);

    // let mut x = 5;
    // let bad_closure = |s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>| { x += 1 };
    // let good_closure = |s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>| { let mut y = 0; y += 1 };
    // group_b.spawn(good_closure, vec![ch3_b_tx], vec![ch3_b_rx]);

    group_a.wait();
    group_b.wait();

}