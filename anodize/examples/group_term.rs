extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};
use crate::anodize::allocator::{BoundedAllocator, add_bound, get_allocated, die, get_bound};
use std::sync::mpsc::{SyncSender, Receiver};
use std::thread::sleep;
use std::time::Duration;


#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}

fn work_1(_s: Vec<SyncSender<i32>>, _r: Vec<Receiver<i32>>) {
    let id = GroupA::get_tag();
    let mut v: Vec<i32> = vec![];
    for i in 1..256 {
        v.push(i);
        println!("{}: Using {}/{} bytes", i, get_allocated(id), get_bound(id));
        sleep(Duration::from_millis(50));
    }
}

fn work_2(_s: Vec<SyncSender<i32>>, _r: Vec<Receiver<i32>>) {
    for i in 1..256 {
        println!("{}: Worker thread", i);
        sleep(Duration::from_millis(50));
    }
}

fn main() {
    add_bound(GroupA::get_tag(), 2048);
    let mut group_a = ThreadGroup::<GroupA>::new();
    group_a.spawn(work_1, vec![], vec![]);
    group_a.spawn(work_2, vec![], vec![]);
    group_a.wait();
    println!("Main thread exiting cleanly!");
    die();
}
