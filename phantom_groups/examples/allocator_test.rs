extern crate phantom_groups;

use crate::phantom_groups::allocator::{BoundedAllocator, add_bound, get_allocated, POOLS_VALID, get_bound};

use std::sync::atomic::{Ordering};
use std::thread::{current};


#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

fn main() {
    let id = current().id();
    add_bound(id, 16384);

    std::thread::spawn(move || {
            // println!("line 14");

        POOLS_VALID.store(true, Ordering::SeqCst);
        let id = current().id();
        add_bound(id, 256);
        let mut v: Vec<i32> = vec![];
        for i in 1..128 {
            v.push(i);
            println!("{}: Using {}/{} bytes", i, get_allocated(id), get_bound(id));
        }
        POOLS_VALID.store(false, Ordering::SeqCst);
    }).join().unwrap();
   
}
