use std::alloc::{GlobalAlloc, Layout};
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread::panicking;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::cell::Cell;

const DEFAULT_BOUND: usize = 4096;
const GROUP_LIMIT: usize = 10;


pub static POOLS_VALID: std::sync::atomic::AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref POOLS: Mutex<HashMap<u64, (AtomicUsize, usize)>> = {
        let mut map = HashMap::new();
        map.reserve(GROUP_LIMIT);
        let pools = Mutex::new(map);
        POOLS_VALID.store(true, Ordering::SeqCst);
        pools
    };
}

thread_local! {
    static IS_OOM: Cell<bool> = Cell::new(false);
    static GROUP: Cell<Option<u64>> = Cell::new(None);
    static IS_DEAD: Cell<bool> = Cell::new(false);
}

pub fn set_group(group: u64) {
    GROUP.with(|cell| { 
        cell.set(Some(group))
    })
}

pub fn get_group() -> Option<u64> {
    GROUP.with(|cell| { 
        cell.get()
    })
}

pub fn die() {
    IS_DEAD.with(|cell| { 
        cell.set(true)
    })
}

pub struct BoundedAllocator { }

impl BoundedAllocator {
    pub const fn new() -> BoundedAllocator {
        BoundedAllocator { }
    }
}

pub fn add_bound(id: u64, bound: usize) {
    let mut locked_pool = POOLS.lock().expect("Mutex failed");
    let a = AtomicUsize::new(0);
    locked_pool.insert(id, (a, bound));
}

pub fn get_allocated(id: u64) -> usize {
    let locked_pool = POOLS.lock().expect("Mutex failed");
    let entry = locked_pool.get(&id);
    match entry {
        Some((atomic, _)) => atomic.load(Ordering::SeqCst),
        None => 0
    }
}

pub fn get_bound(id: u64) -> usize {
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
        } else if panicking() {
           System.alloc(_layout)
        } else {
            
            match get_group() {
                None => System.alloc(_layout),
                Some(id) => {
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
                        IS_OOM.with(|x| {x.set(true)});
                        None
                    } else {
                        Some(System.alloc(_layout))
                    }
                }.expect("Thread out of memory")
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        if !POOLS_VALID.load(Ordering::SeqCst) {
            System.dealloc(_ptr, _layout)
        } else if panicking() {
            System.dealloc(_ptr, _layout)
        } else { 
            match get_group() {
                None => System.dealloc(_ptr, _layout),
                Some(id) => {
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
    }
}
