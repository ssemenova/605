use std::alloc::{GlobalAlloc, Layout};
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread::{current, panicking, ThreadId};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::panic;
use std::cell::Cell;

const DEFAULT_BOUND: usize = 4096;
const THREAD_LIMIT: usize = 10;


pub static POOLS_VALID: std::sync::atomic::AtomicBool = AtomicBool::new(false);

lazy_static! {
    pub static ref PRIMORDIAL_TID: ThreadId = current().id();
    static ref POOLS: Mutex<HashMap<ThreadId, (AtomicUsize, usize)>> = {
        let mut map = HashMap::new();
        map.reserve(THREAD_LIMIT);
        let pools = Mutex::new(map);
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

pub fn add_bound(id: ThreadId, bound: usize) {
    let mut locked_pool = POOLS.lock().expect("Mutex failed");
    let a = AtomicUsize::new(0);
    locked_pool.insert(id, (a, bound));
}

pub fn get_allocated(id: ThreadId) -> usize {
    let locked_pool = POOLS.lock().expect("Mutex failed");
    let entry = locked_pool.get(&id);
    match entry {
        Some((atomic, _)) => atomic.fetch_add(0, Ordering::SeqCst),
        None => 0
    }
}

pub fn get_bound(id: ThreadId) -> usize {
    let locked_pool = POOLS.lock().expect("Mutex failed");
    let entry = locked_pool.get(&id);
    match entry {
        Some((_, bound)) => *bound,
        None => 0
    }
}

unsafe impl GlobalAlloc for BoundedAllocator {
    
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // return System.alloc(_layout);
        if !POOLS_VALID.load(Ordering::SeqCst) {
            System.alloc(_layout)
        } else if panicking() || IS_OOM.with(|x| {x.get()}) {
           System.alloc(_layout)
        } else {
            // let id = panic::catch_unwind(|| { current().id() });
            let id: Result<ThreadId, ()> = Ok( current().id() );
            let bypass = id.is_err() || id.unwrap() == *PRIMORDIAL_TID;
            
            if bypass {
                // print!("P: {:?}", current().id());
                System.alloc(_layout)
            } else {
                let id = current().id();
                // print!("C: {:?}", id);

                let result = {

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
                };

                result.expect("Thread out of memory")
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // println!("{:#x?}", _ptr);
        if !POOLS_VALID.load(Ordering::SeqCst) {
            System.dealloc(_ptr, _layout)
        } else if panicking() || IS_OOM.with(|x| {x.get()}) {
            System.dealloc(_ptr, _layout)
        } else { 
            // let id = panic::catch_unwind(|| { current().id() });
            let id: Result<ThreadId, ()> = Ok( current().id() );
            let bypass = id.is_err() || id.unwrap() == *PRIMORDIAL_TID;
            
            if bypass {
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
}
