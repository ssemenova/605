extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};
use crate::anodize::allocator::{BoundedAllocator, add_bound, get_allocated, die, get_bound};
use std::sync::mpsc::{Sender, Receiver};

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}

fn work(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let id = GroupA::get_tag();
    let mut v: Vec<i32> = vec![];
    for i in 1..256 {
        v.push(i);
        println!("{}: Using {}/{} bytes", i, get_allocated(id), get_bound(id));
    }
}

fn main() {
    add_bound(GroupA::get_tag(), 2048);
    let mut group_a = ThreadGroup::<GroupA>::new();
    group_a.spawn(work, vec![], vec![]);
    group_a.wait();
    println!("Main thread exiting cleanly!");
    die();
}
