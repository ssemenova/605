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
extern crate ncollide3d;
extern crate device_query;

use crate::anodize::thread_groups::{GroupTag, ThreadGroup};
use crate::anodize::allocator::{BoundedAllocator, add_bound};

use serde::{Serialize, Deserialize};

use std::sync::mpsc::{Sender, Receiver};
use std::{path::Path, fs::File};

//use rclrs;
//use std_msgs;

pub mod visodom;
pub mod minimal_planner;
pub mod teleop;

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();

#[derive(Serialize, Deserialize)]
pub struct Settings {
    freiberg_type: String,
    path_to_associations: String,
    path_to_urdf: String,
    turtlebot3_model: String,
    max_lin_vel: String,
    max_ang_vel: String,
    lin_vel_step_size: String,
    angle_vel_step_size: String
}

struct VisodomGroup;
impl GroupTag for VisodomGroup { 
    fn get_tag() -> u64 { 0x41 }
}

struct PublisherGroup;
impl GroupTag for PublisherGroup {
    fn get_tag() -> u64 {0x10}
}

struct PlannerGroup;
impl GroupTag for PlannerGroup {
    fn get_tag() -> u64 {0x20}
}

struct TeleopGroup;
impl GroupTag for TeleopGroup {
    fn get_tag() -> u64 {0x30}
}

fn main() {
    add_bound(VisodomGroup::get_tag(), usize::MAX / 4);
    add_bound(PublisherGroup::get_tag(), usize::MAX / 4);
    add_bound(PlannerGroup::get_tag(), usize::MAX / 4);
    add_bound(TeleopGroup::get_tag(), usize::MAX / 4);
    
    let mut visodom_group = ThreadGroup::<VisodomGroup>::new();
    let mut publisher_group = ThreadGroup::<PublisherGroup>::new();
    let mut planner_group = ThreadGroup::<PlannerGroup>::new();
    let mut teleop_group = ThreadGroup::<TeleopGroup>::new();

    visodom_group.spawn(run_visodom, vec![], vec![]);
    publisher_group.spawn(run_publisher, vec![], vec![]);
    planner_group.spawn(run_planner, vec![], vec![]);
    teleop_group.spawn(run_teleop, vec![], vec![]);
    
    visodom_group.wait();
    publisher_group.wait();
    planner_group.wait();
    teleop_group.wait();
}

fn run_publisher(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let context = rclrs::Context::default();

    let node = context.create_node("minimal_publisher");
    
    let node = match node {
    	Ok(node) => node,
    	Err(_e) => return,
    };

    let publisher =
        node.create_publisher::<std_msgs::msg::String>("start_planner", rclrs::QOS_PROFILE_DEFAULT);
        
    let publisher = match publisher {
    	Ok(publisher) => publisher,
    	Err(_e) => return,
    };

    let mut message = std_msgs::msg::String::default();

    let mut publish_count: u32 = 1;

    while context.ok() {
        publisher.publish(&message);
        
        publish_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(5500));
    }
}

 fn run_planner(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let context = rclrs::Context::default();

    let node = context.create_node("planner");
    
    let mut node = match node {
    	Ok(node) => node,
    	Err(_e) => return,
    };

    let mut num_messages: usize = 0;
    
    let config = read_config_file();

    let _subscription = node.create_subscription::<std_msgs::msg::String, _>(
        "start_planner",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: &std_msgs::msg::String| {
            println!("Planner thread starting");
            minimal_planner::main(config.path_to_urdf.clone());
            num_messages += 1;
        },
    );
    
    let result = rclrs::spin(&node);
    match result {
    	Ok(_r) => println!("Spin once"),
    	Err(_e) => return,
    };
}

fn run_teleop(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let config = read_config_file();

    println!("Teleop thread starting");

    teleop::teleop_entrypoint(config);
}

fn run_visodom(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    let config = read_config_file();

    println!("Visual odometry thread starting");

    let result = visodom::visodom_entrypoint::run(config);
    match result {
        Ok(_v) => println!("visual odometry finished"),
        Err(e) => println!("visual odometry failed with error: {:?}", e),
    }
}

fn read_config_file() -> Settings {
    let json_file_path = Path::new("config.json");
    let json_file = File::open(json_file_path).expect("config file not found");
    let settings: Settings =
        serde_json::from_reader(json_file).expect("error while reading config file");
    
    settings
}
