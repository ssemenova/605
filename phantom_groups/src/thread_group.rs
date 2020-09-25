use std::marker::PhantomData;
use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{Sender, Receiver, channel};

pub trait GroupTag { }
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

    pub fn spawn<F>(&mut self, f: F, senders: Vec<IntragroupSender<i32, G>>, receivers: Vec<IntragroupReceiver<i32, G>>) -> ()
    where
        F: FnOnce(Vec<Sender<i32>>, Vec<Receiver<i32>>) -> (),
        F: Send + 'static,
    {
        // ...
        let s = senders.into_iter().map(move |e| e.sender).collect();
        let r = receivers.into_iter().map(move |e| e.receiver).collect();
        let join = spawn(move || { f(s, r) });
        self.threads.push(join);
        ()
    }
}


