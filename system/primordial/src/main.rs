#![allow(unused_imports)]
#![allow(unused_variables)]

/*
Primordial thread
*/
extern crate anodize;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup, TaggedThread};

use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
use std::{env, error::Error, fs, io::BufReader, io::Read, path::Path, path::PathBuf};

mod visodom_entrypoint;

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut group_a = ThreadGroup::<GroupA>::new();


    let visodom_thread = TaggedThread::new(visodom_entrypoint::run);

    
    group_a.spawn_thread(visodom_thread);

    group_a.wait();
}
