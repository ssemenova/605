use std::thread;
pub struct GroupedThread {
  pub group: i32,
  
}

impl GroupedThread {
  pub fn new(group: i32) -> GroupedThread {
    let GroupedThread = GroupedThread{
      group
    };

    return GroupedThread;
  }


  pub fn spawn() {
    
  }
}