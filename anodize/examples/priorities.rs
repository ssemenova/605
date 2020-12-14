/*
Commented-out examples of code that breaks the library and is checked at compile time.
*/
extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup, set_prio};

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
    set_prio(99);

    let mut group_one = ThreadGroup::<GroupOne>::new();
    let mut group_two = ThreadGroup::<GroupTwo>::new();
    let mut group_three = ThreadGroup::<GroupThree>::new();

    println!("Spawning group 1");
    // group_one.spawn(work_one, vec!(), vec!());
    // sleep(Duration::from_secs(1));
    // println!("Spawning group 2");
    // group_two.spawn(work_two, vec!(), vec!());
    // sleep(Duration::from_secs(1));
    // println!("Spawning group 3");
    // group_three.spawn(work_three, vec!(), vec!());
    // sleep(Duration::from_secs(1));
    
    println!("Spawning group 3");
    group_three.spawn(work_three, vec!(), vec!());
    sleep(Duration::from_secs(1));
    println!("Spawning group 2");
    group_two.spawn(work_two, vec!(), vec!());
    sleep(Duration::from_secs(1));
    println!("Spawning group 1");
    group_one.spawn(work_one, vec!(), vec!());
    sleep(Duration::from_secs(1));

}

fn work_one(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = s;
    let _ = r;

    loop {
        println!("Thread {}", 1);
        let mut _a: u64 = 0;
        for i in 1..999999 {
            _a += i;
        }
    }
}

fn work_two(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = s;
    let _ = r;

    loop {
        println!("Thread {}", 2);
        let mut _a: u64 = 0;
        for i in 1..999999 {
            _a += i;
        }
    }
}

fn work_three(s: Vec<SyncSender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = s;
    let _ = r;

    loop {
        println!("Thread {}", 3);
        let mut _a: u64 = 0;
        for i in 1..999999 {
            _a += i;
        }
    }
}
