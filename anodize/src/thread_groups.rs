use std::marker::PhantomData;
use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::allocator::set_group;
use libc;

type Thunk = fn(Vec<Sender<i32>>, Vec<Receiver<i32>>) -> ();

pub trait GroupTag { fn get_tag() -> u64; fn get_prio() -> i32 { return 1; } }
trait GroupMember<G: GroupTag> { }

#[derive(Clone)]
pub struct IntragroupSender<V, G: GroupTag> {
    sender: Sender<V>,
    marker: PhantomData<G>,
}

impl<V,G: GroupTag> GroupMember<G> for IntragroupSender<V, G> { }

pub struct IntragroupReceiver<V, G: GroupTag> {
    receiver: Receiver<V>,
    marker: PhantomData<G>,
}

impl<V, G: GroupTag> GroupMember<G> for IntragroupReceiver<V, G> { }

pub struct ThreadGroup<G: GroupTag> {
    threads: Vec<JoinHandle<()>>,
    marker: PhantomData<G>,
}

pub struct TaggedThread<G: GroupTag> {
    thunk: Thunk,
    sends: Vec<IntragroupSender<i32, G>>,
    recvs: Vec<IntragroupReceiver<i32, G>>,
    marker: PhantomData<G>,
}

impl<G: GroupTag> TaggedThread<G> {
    pub fn new(f: Thunk) -> TaggedThread<G> 
    where
    Thunk: Send + 'static
    {
        TaggedThread { thunk: f, sends: vec![], recvs: vec![], marker: PhantomData }
    }

    fn add_send(mut self, sender: IntragroupSender<i32, G>) -> TaggedThread<G> {
        self.sends.push(sender);
        self
    }

    fn add_recv(mut self, receiver: IntragroupReceiver<i32, G>) -> TaggedThread<G> {
        self.recvs.push(receiver);
        self
    }
}

impl<G: GroupTag> ThreadGroup<G> 
{
    pub fn new() -> ThreadGroup<G> {
        ThreadGroup { threads: vec![], marker: PhantomData }
    }

    pub fn channel<V>(&self) -> (IntragroupSender<V, G>, IntragroupReceiver<V, G>)
    {
        let (s, r) = channel ();
        (IntragroupSender { sender: s, marker: PhantomData }, IntragroupReceiver { receiver: r, marker: PhantomData })
    }

    pub fn link(&self, from: TaggedThread<G>, to: TaggedThread<G>) -> (TaggedThread<G>, TaggedThread<G>) {
        let (s, r) = self.channel::<i32>();
        (from.add_send(s), to.add_recv(r))
    }

    pub fn spawn_thread(&mut self, t: TaggedThread<G>) -> () {
        self.spawn(t.thunk, t.sends, t.recvs)
    }

    pub fn spawn(&mut self, f: Thunk, senders: Vec<IntragroupSender<i32, G>>, receivers: Vec<IntragroupReceiver<i32, G>>) -> ()
    where
        Thunk: Send + 'static,
    {
        // ...
        let s = senders.into_iter().map(move |e| e.sender).collect();
        let r = receivers.into_iter().map(move |e| e.receiver).collect();
        let join = spawn(move || { 
            set_group(G::get_tag()); 
            unsafe {
                let pid = libc::pthread_self();
                let policy = libc::SCHED_FIFO;
                let max = libc::sched_get_priority_max(policy);
                let min = libc::sched_get_priority_min(policy);
                println!("prio range =  [{}, {}]", min, max);
                let prio = G::get_prio();
                assert!(prio >= min, "prio < min");
                assert!(prio <= max, "prio > max");
                let param = libc::sched_param { sched_priority: prio };
                let err = libc::pthread_setschedparam(pid, policy, &param);
                assert!(err == 0, "err = {}", err);
            }
            let ret = f(s, r); ret 
        });
        self.threads.push(join);
        ()
    }

    pub fn wait(self) -> ()
    {
        self.threads.into_iter().for_each(|h| {let _ = h.join();})
    }
}


