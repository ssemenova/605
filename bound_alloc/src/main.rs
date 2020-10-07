use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};

struct BoundedAllocator {
    total_alloc: AtomicUsize,
    alloc_bound: usize, 
}

impl BoundedAllocator {
    const fn new(bound: usize) -> BoundedAllocator {
        BoundedAllocator {
            total_alloc: AtomicUsize::new(0),
            alloc_bound: bound
        }
    }
}

unsafe impl GlobalAlloc for BoundedAllocator {
    
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let total = self.total_alloc.fetch_add(_layout.size(), Ordering::SeqCst);
        if total + _layout.size() > self.alloc_bound {
            self.total_alloc.fetch_sub(_layout.size(), Ordering::SeqCst);
            null_mut()
        } else {
            System.alloc(_layout)
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        self.total_alloc.fetch_sub(_layout.size(), Ordering::SeqCst);
        System.dealloc(_ptr, _layout)
    }
}

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new(6144);

fn main() {
    let mut v = vec![];
    for i in 1..1024 {
        v.push(i);
        println!("{}: Using {}/{} bytes", i, 
                    A.total_alloc.fetch_add(0, Ordering::SeqCst), 
                    A.alloc_bound);
    }
}
