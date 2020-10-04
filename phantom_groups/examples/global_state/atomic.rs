#[allow(unused_variables)]

extern crate phantom_groups;

use std::sync::mpsc::{Sender, Receiver};
use crate::phantom_groups::{GroupTag, ThreadGroup, TaggedThread};
use std::thread::sleep;
use std::time::Duration;

use std::sync::atomic::{AtomicUsize, Ordering};
static COUNT: AtomicUsize = AtomicUsize::new(0);

struct GroupA;
impl GroupTag for GroupA { }

struct GroupB;
impl GroupTag for GroupB { }


fn isolation_breaker(s: Vec<Sender<i32>>, r: Vec<Receiver<i32>>) {

    let _ = (s, r);

    loop {
        COUNT.fetch_add(1, Ordering::SeqCst);
        sleep(Duration::from_secs(1));
    }
}

fn main() {

    let mut group_a = ThreadGroup::<GroupA>::new();

    let (ch1_a_tx, ch1_a_rx) = group_a.channel::<i32>();
    let (ch2_a_tx, ch2_a_rx) = group_a.channel::<i32>();

    group_a.spawn(isolation_breaker, vec![ch1_a_tx], vec![ch2_a_rx]);

    group_a.spawn(isolation_breaker, vec![], vec![]);

    group_a.wait();

}
