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

pub mod visodom;

#[global_allocator]
static A: BoundedAllocator = BoundedAllocator::new();


#[derive(Serialize, Deserialize)]
struct VisOdomSettings {
    version: String,
    freiberg_type: String,
    path_to_associations: String,
}

struct GroupA;
impl GroupTag for GroupA { 
    fn get_tag() -> u64 { 0x41 }
}

fn main() {
    add_bound(GroupA::get_tag(), usize::MAX / 2);
    let mut group_a = ThreadGroup::<GroupA>::new();
    group_a.spawn(run_visodom, vec![], vec![]);
    group_a.wait();
}

fn run_visodom(_s: Vec<Sender<i32>>, _r: Vec<Receiver<i32>>) {
    // Code to read config from json file
    // Might need later ... don't want to mess with what we have working now
    // let json_file_path = Path::new("../config.json");
    // let json_file = File::open(json_file_path).expect("config file not found");
    // let settings: VisOdomSettings =
    //     serde_json::from_reader(json_file).expect("error while reading json");
    let args = [String::from("fr1"), String::from("/Users/sofiya/rgbd_dataset_freiburg1_desk2/associations.txt")];

    let result = visodom::visodom_entrypoint::run(&args);
    match result {
        Ok(_v) => println!("visual odometry finished"),
        Err(e) => println!("visual odometry failed with error: {:?}", e),
    }
}