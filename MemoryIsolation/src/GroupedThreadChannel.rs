use crate::GroupedThread::GroupedThread;
use std::sync::mpsc::channel;
use std::thread;

pub struct GroupedThreadChannel {
}

impl GroupedThreadChannel {
  pub fn new(group: i32) -> GroupedThreadChannel {
    let GroupedThread = GroupedThreadChannel{};

    return GroupedThread;
  }


  pub fn send(t1 : GroupedThread, t2: GroupedThread) {
    
  }
}