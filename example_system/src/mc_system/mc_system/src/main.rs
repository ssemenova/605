extern crate anodize;
extern crate serde;
extern crate serde_json;
extern crate nalgebra;
extern crate num_traits;
extern crate rand;
extern crate itertools;
extern crate nom;
extern crate byteorder;
extern crate png;
extern crate image;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};
use crate::anodize::allocator::{BoundedAllocator, add_bound};

use serde::{Serialize, Deserialize};

use std::sync::mpsc::{Sender, Receiver};
use std::{path::Path, fs::File};

//use rclrs;
//use std_msgs;

pub mod visodom;

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

#[derive(Serialize, Deserialize)]
struct VisOdomSettings {
    freiberg_type: String,
    path_to_associations: String,
}

struct VisodomGroup;
impl GroupTag for VisodomGroup { 
    fn get_tag() -> u64 { 0x41 }
}

struct PublisherGroup;
impl GroupTag for PublisherGroup {
    fn get_tag() -> u64 {0x10}
}

struct SubscriberGroup;
impl GroupTag for SubscriberGroup {
    fn get_tag() -> u64 {0x30}
}

fn main() {
    add_bound(VisodomGroup::get_tag(), usize::MAX / 3);
    add_bound(PublisherGroup::get_tag(), usize::MAX / 3);
    add_bound(SubscriberGroup::get_tag(), usize::MAX / 3);
    
    let mut visodom_group = ThreadGroup::<VisodomGroup>::new();
    let mut publisher_group = ThreadGroup::<PublisherGroup>::new();
    let mut subscriber_group = ThreadGroup::<SubscriberGroup>::new();
    
    visodom_group.spawn(run_visodom, vec![], vec![]);
    subscriber_group.spawn(run_subscriber, vec![], vec![]);
    publisher_group.spawn(run_publisher, vec![], vec![]);
    
    visodom_group.wait();
    subscriber_group.wait();
    publisher_group.wait();
}

fn run_publisher(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let context = rclrs::Context::default();

    let node = context.create_node("minimal_publisher");
    
    let mut node = match node {
    	Ok(node) => node,
    	Err(node) => return,
    };

    let publisher =
        node.create_publisher::<std_msgs::msg::String>("topic", rclrs::QOS_PROFILE_DEFAULT);
        
    let mut publisher = match publisher {
    	Ok(publisher) => publisher,
    	Err(publisher) => return,
    };

    let mut message = std_msgs::msg::String::default();

    let mut publish_count: u32 = 1;

    while context.ok() {
        message.data = format!("Hello, world! {}", publish_count);
        println!("Publishing: [{}]", message.data);
        publisher.publish(&message);
        //match result {
        //	Ok(result) => println!("Publishing: [{}]", message.data),
        //	Err(result) => println!("Could not publish [{}]", message.data),
        //};
        
        publish_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn run_subscriber(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let context = rclrs::Context::default();

    let mut node = context.create_node("minimal_subscriber");
    
    let mut node = match node {
    	Ok(node) => node,
    	Err(node) => return,
    };

    let mut num_messages: usize = 0;

    let _subscription = node.create_subscription::<std_msgs::msg::String, _>(
        "topic",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: &std_msgs::msg::String| {
            println!("I heard: '{}'", msg.data);
            println!("(Got {} messages so far)", num_messages);
            num_messages += 1;
        },
    );

    let result = rclrs::spin(&node);
    match result {
    	Ok(result) => println!("Spin once"),
    	Err(result) => return,
    };
}

fn run_visodom(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let json_file_path = Path::new("config.json");
    let json_file = File::open(json_file_path).expect("config file not found");
    let settings: VisOdomSettings =
        serde_json::from_reader(json_file).expect("error while reading config file");
    let args = [settings.freiberg_type, settings.path_to_associations];

    println!("Visual odometry thread starting");

    let result = visodom::visodom_entrypoint::run(&args);
    match result {
        Ok(_v) => println!("visual odometry finished"),
        Err(e) => println!("visual odometry failed with error: {:?}", e),
    }
}
