mod thread_group;
mod hlist;

use std::sync::mpsc::{Sender, Receiver};
use crate::thread_group::{GroupTag, ThreadGroup};
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

    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();

    let (ch1_a_tx, ch1_a_rx) = group_a.channel::<i32>();
    let (ch2_a_tx, ch2_a_rx) = group_a.channel::<i32>();

    let (ch1_b_tx, ch1_b_rx) = group_b.channel::<i32>();
    let (ch2_b_tx, ch2_b_rx) = group_b.channel::<i32>();

    group_a.spawn(produce, vec![ch1_a_tx], vec![ch2_a_rx]);
    group_a.spawn(consume, vec![ch2_a_tx], vec![ch1_a_rx]);

    group_b.spawn(produce, vec![ch1_b_tx], vec![ch2_b_rx]);
    group_b.spawn(consume, vec![ch2_b_tx], vec![ch1_b_rx]);

    group_a.wait();
    group_b.wait();
}
