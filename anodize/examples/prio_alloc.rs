extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};
use crate::anodize::allocator::{BoundedAllocator, add_bound, get_allocated, die, get_bound};
use std::sync::mpsc::{SyncSender, Receiver};

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
    fn get_prio() -> i32 { 0x01 }
}

struct GroupB;
impl GroupTag for GroupB { 
    fn get_tag() -> u64 { 0x42 }
    fn get_prio() -> i32 { 0x02 }
}

fn work_a(_s: Vec<SyncSender<i32>>, _r: Vec<Receiver<i32>>) {
    for _ in 1..64 {
        println!("Thread A");
        let mut _a: u64 = 0;
        for i in 1..999999 {
            _a += i;
        }
    }
}

fn work_b(_s: Vec<SyncSender<i32>>, _r: Vec<Receiver<i32>>) {
    let id = GroupA::get_tag();
    let mut v: Vec<i32> = vec![];
    for i in 1..1024 {
        v.push(i);
        println!("{}: Using {}/{} bytes", i, get_allocated(id), get_bound(id));
    }
}

fn main() {
    add_bound(GroupA::get_tag(), 2048);
    
    let mut group_a = ThreadGroup::<GroupA>::new();
    group_a.spawn(work_a, vec![], vec![]);
    
    let mut group_b = ThreadGroup::<GroupB>::new();
    group_b.spawn(work_b, vec![], vec![]);
    
    group_b.wait();
    group_a.wait();
    println!("Main thread exiting cleanly!");
    die();
}
