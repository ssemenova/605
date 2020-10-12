extern crate phantom_groups;

use phantom_groups::thread_groups::GroupTag;
use crate::phantom_groups::allocator::{BoundedAllocator, add_bound, get_allocated, POOLS_VALID, get_bound, PRIMORDIAL_TID};

use std::sync::atomic::{Ordering};
use std::thread::{park, current};
use std::panic;


#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}

fn work() {
    println!("line 14");

    POOLS_VALID.store(true, Ordering::SeqCst);
    let id = current().id();
    add_bound(id, 256);
    let mut v: Vec<i32> = vec![];
    for i in 1..128 {
        v.push(i);
        println!("{}: Using {}/{} bytes", i, get_allocated(id), get_bound(id));
    }
    POOLS_VALID.store(false, Ordering::SeqCst);
}

fn main() {

    // panic::set_hook(Box::new(|i| { println!("{:#?}", i) }));
    

    let id = *PRIMORDIAL_TID;
    add_bound(id, 16384);

    std::thread::spawn(work).join();
    println!("Main thread exiting cleanly!");
    POOLS_VALID.store(false, Ordering::SeqCst);
}
