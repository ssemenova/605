/*
Example of breaking our code with global state.
A global AtomicUSize is created and can be accessed/incremented 
by two threads in two different groups.
*/
extern crate phantom_groups;

use crate::phantom_groups::thread_groups::{GroupTag, ThreadGroup};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;

use std::sync::atomic::{AtomicUsize, Ordering};
static COUNT: AtomicUsize = AtomicUsize::new(0);

struct GroupA;
impl GroupTag for GroupA { }

struct GroupB;
impl GroupTag for GroupB { }


fn main() {
    let mut group_a = ThreadGroup::<GroupA>::new();
    let mut group_b = ThreadGroup::<GroupB>::new();

    group_a.spawn(counter, vec![], vec![]);
    group_b.spawn(counter, vec![], vec![]);

    group_a.wait();
    group_b.wait();
}


fn counter(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {
    let _ = (s, r);

    loop {
        COUNT.fetch_add(1, Ordering::SeqCst);
        sleep(Duration::from_secs(1));
    }
}
