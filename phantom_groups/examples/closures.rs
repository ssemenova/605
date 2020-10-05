/*
An example of passing acceptable and unnacceptable closures to TaggedThreads.
*/
extern crate phantom_groups;

use crate::phantom_groups::{GroupTag, ThreadGroup, TaggedThread};

use std::sync::mpsc::{Sender, Receiver};


struct GroupA;
impl GroupTag for GroupA { }


fn main() {
    good_closure();
    bad_closure();
}


fn good_closure() {
    let mut group_a = ThreadGroup::<GroupA>::new();


    // This closure does not capture any variables from the scope, so it is fine to use with TaggedThread.
    let mut _x = 5;
    let good_closure = |_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>| { let mut _y = 0; _y += 1 };


    let group_a_t1 = TaggedThread::new(good_closure);
    group_a.spawn_thread(group_a_t1);
}


fn bad_closure() {
    let _group_a = ThreadGroup::<GroupA>::new();

    // This closure DOES capture the variable `x` from the scope, so it will not work with TaggedThread.
    let mut x = 5;
    let _bad_closure = |_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>| { x += 1 };


    /*
        Using bad_closures gives the compiler error:
            mismatched types
            expected fn pointer `fn(std::vec::Vec<std::sync::mpsc::Sender<i32>>, std::vec::Vec<std::sync::mpsc::Receiver<i32>>)`
            found closure `[closure@examples/closures.rs:39:23: 39:78 x:_]`
    */
    // let group_a_t1 = TaggedThread::new(_bad_closure);
    // _group_a.spawn_thread(group_a_t1);
}