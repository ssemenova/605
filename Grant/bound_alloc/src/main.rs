use std::alloc::{GlobalAlloc, Layout};
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread::{current, ThreadId};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

const DEFAULT_BOUND: usize = 4096;

static POOLS_VALID: std::sync::atomic::AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref PRIMORDIAL_TID: ThreadId = current().id();
    static ref POOLS: Mutex<HashMap<ThreadId, (AtomicUsize, usize)>> = {
        let pools = Mutex::new(HashMap::new());
        pools
    };
}

struct BoundedAllocator { }

impl BoundedAllocator {
    const fn new() -> BoundedAllocator {
        BoundedAllocator { }
    }
}

fn add_bound(id: ThreadId, bound: usize) {
    let mut locked_pool = POOLS.lock().expect("Mutex failed");
    let a = AtomicUsize::new(0);
    locked_pool.insert(id, (a, bound));
}

fn get_allocated(id: ThreadId) -> usize {
    let locked_pool = POOLS.lock().expect("Mutex failed");
    let entry = locked_pool.get(&id);
    match entry {
        Some((atomic, _)) => atomic.fetch_add(0, Ordering::SeqCst),
        None => 0
    }
}

fn get_bound(id: ThreadId) -> usize {
    let locked_pool = POOLS.lock().expect("Mutex failed");
    let entry = locked_pool.get(&id);
    match entry {
        Some((_, bound)) => *bound,
        None => 0
    }
}

unsafe impl GlobalAlloc for BoundedAllocator {
    
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {

        if !POOLS_VALID.load(Ordering::SeqCst) {
            System.alloc(_layout)
        } else if current().id() == *PRIMORDIAL_TID {
            println!("P: {:?}", current().id());
            System.alloc(_layout)
        } else {
            let id = current().id();
            println!("C: {:?}", id);
            let mut locked_pool = POOLS.lock().expect("Mutex failed");

            let entry = locked_pool.get(&id);

            let (total_alloc, bound) = match entry {
                Some((a, b)) => (a, *b),
                None => {
                    let a = AtomicUsize::new(0);
                    let b = DEFAULT_BOUND;
                    locked_pool.insert(id, (a,b));
                    let (a, b) = locked_pool.get(&id).unwrap();
                    (a, *b)
                }
            };

            let total = total_alloc.fetch_add(_layout.size(), Ordering::SeqCst);
            if total + _layout.size() > bound {
                total_alloc.fetch_sub(_layout.size(), Ordering::SeqCst);
                // null_mut()
                panic!("Out of memory")
            } else {
                System.alloc(_layout)
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

        if !POOLS_VALID.load(Ordering::SeqCst) {
            System.dealloc(_ptr, _layout)
        } else if current().id() == *PRIMORDIAL_TID {
            System.dealloc(_ptr, _layout)
        } else {
            let id = current().id();
            let mut locked_pool = POOLS.lock().expect("Mutex failed");
            let entry = locked_pool.get(&id);
            let (total_alloc, _) = match entry {
                Some((a, b)) => (a, *b),
                None => {
                    // Uh, what?
                    // Init with size so it ends up at zero
                    let a = AtomicUsize::new(_layout.size());
                    let b = DEFAULT_BOUND;
                    locked_pool.insert(id, (a,b));
                    let (a, b) = locked_pool.get(&id).unwrap();
                    (a, *b)
                }
            };

            total_alloc.fetch_sub(_layout.size(), Ordering::SeqCst);
            System.dealloc(_ptr, _layout)
        }
    }
}

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

fn main() {
    let id = current().id();
    add_bound(id, 16384);
    std::thread::spawn(move || {
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
