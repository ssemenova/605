extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};

use std::sync::mpsc::{SyncSender, Receiver};
use std::thread::sleep;
use std::time::Duration;


struct GroupOne;
impl GroupTag for GroupOne { 
    fn get_tag() -> u64 { 0x01 }
    fn get_prio() -> i32 { 0x01 }
}

struct GroupTwo;
impl GroupTag for GroupTwo { 
    fn get_tag() -> u64 { 0x02 }
    fn get_prio() -> i32 { 0x02 }
}

struct GroupThree;
impl GroupTag for GroupThree { 
    fn get_tag() -> u64 { 0x03 }
    fn get_prio() -> i32 { 0x03 }
}

fn main() {
    let mut group_one = ThreadGroup::<GroupOne>::new();
    let mut group_two = ThreadGroup::<GroupTwo>::new();
    let mut group_three = ThreadGroup::<GroupThree>::new();
    
    let (t1, r1) = group_one.channel::<i32>();
    let (t2, r2) = group_two.channel::<i32>();
    let (t3, r3) = group_three.channel::<i32>();

    println!("Spawning group 1");
    group_one.spawn(produce_one, vec!(t1), vec!());
    group_one.spawn(consume, vec!(), vec!(r1));
    sleep(Duration::from_secs(2));
    println!("Spawning group 2");
    group_two.spawn(produce_two, vec!(t2), vec!());
    group_two.spawn(consume, vec!(), vec!(r2));
    sleep(Duration::from_secs(2));
    println!("Spawning group 3");
    group_three.spawn(produce_three, vec!(t3), vec!());
    group_three.spawn(consume, vec!(), vec!(r3));
    sleep(Duration::from_secs(2));
}

fn produce_one(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let _ = r;

    loop {
        let _ = tx.send(1);
    }
}

fn produce_two(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let _ = r;

    loop {
        let _ = tx.send(2);
    }
}

fn produce_three(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let tx = s.get(0).unwrap();
    let _ = r;

    loop {
        let _ = tx.send(3);
    }
}

fn consume(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = s;
    let rx = r.get(0).unwrap();

    loop {
        let i = rx.recv().unwrap();
        println!("Consuming {}", i);
        let mut a: u64 = 0;
        for i in 1..999999 {
            a += i;
        }
        let _ = a;
        //println!("{}", a);
    }
}
