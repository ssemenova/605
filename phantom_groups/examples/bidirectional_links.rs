/*
Example of bi-directional links between threads in one thread group.
Uses mostly the same setup as simple_with_links.
*/
extern crate phantom_groups;

use crate::phantom_groups::{GroupTag, ThreadGroup, TaggedThread};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;


struct GroupA;
impl GroupTag for GroupA { }


fn main() {
    // Create two groups
    let mut group_a = ThreadGroup::<GroupA>::new();

    let group_a_t1 = TaggedThread::new(thread1_function);
    let group_a_t2 = TaggedThread::new(thread2_function);

    // Link t1 and t2. This creates an mpsc channel from t1->t2
    let (group_a_t1, group_a_t2) = group_a.link(group_a_t1, group_a_t2);

    // Link t2 to t1, creating an mpsc channel from t2->t1
    let (group_a_t2, group_a_t1) = group_a.link(group_a_t2, group_a_t1);

    group_a.spawn_thread(group_a_t1);
    group_a.spawn_thread(group_a_t2);

    group_a.wait();
}


fn thread1_function(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let rx = r.get(0).unwrap();

    let mut send_count = 0;
    loop {
        println!("T1 sending {}", send_count);
        let _ = tx.send(send_count);
        send_count += 1;

        let recv_count = rx.recv().unwrap();
        println!("T1 consuming {}", recv_count);

        sleep(Duration::from_secs(1));
    }
}


fn thread2_function(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let rx = r.get(0).unwrap();

    let mut send_count = 0;
    loop {
        println!("T2 sending {}", send_count);
        let _ = tx.send(send_count);
        send_count += 1;

        let recv_count = rx.recv().unwrap();
        println!("T2 consuming {}", recv_count);

        sleep(Duration::from_secs(1));
    }
}
