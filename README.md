# Anodize: Language-Level Guarantees for Mixed Criticality Systems

## Running example mixed criticality system

The example mixed criticality system is run in the `example_system` subdirectory. This program contains the *primordial thread* that configures the system and spawns each "program" in the mixed criticality system. The programs are:

1. A visual odometry program that reads a SLAM dataset and outputs the location of the robot at each time step. This code is written in *pure Rust* and is, for the most part, unchanged code from [this github repository](https://github.com/mpizenberg/visual-odometry-rs). It can be found in `system/src/visodom`.

2. A trivial ROS publisher

3. A trivial ROS subscriber

While the example mixed criticality system largely lives in the `example_system` subdirectory of this repository, it has a crate dependency to *anodize* (within the `anodize` subdirectory of this repository). Further, our example system relies on [ros2_rust](https://github.com/ros2-rust/ros2_rust), a library that creates Rust bindings to ROS C++ code. Thus, the easiest way to get the system running is to download this entire repository and follow the steps below to configure each part of the system.

There are several components of the example mixed criticality system that need to be configured:

1. Getting ROS2 running on your computer
2. Downloading an RGBD SLAM dataset and converting it into an associations file
3. Modifying `mc_system`'s configuration file to point to the dataset
4. Modifying the Cargo file to point to the anodize library

We tested our system with Ubuntu 20 and ROS2 foxy. Technically, you should be able to use Ubuntu 18 and ROS2 eloquent, but it will require some effort to set up. See [here](https://github.com/ros2-rust/ros2_rust/issues/19) and [here](https://github.com/ros2-rust/ros2_rust/issues/21). Andrew's fork of the ros2_rust repository for eloquent support [here](https://github.com/Wisc-HCI/ros2_rust) may be useful, but (assuming it works) you would have to replace all the ros2_rust code in this repository with his.

#### Note: ROS2 <-> Rust bindings

We use the repository [ros2_rust](https://github.com/ros2-rust/ros2_rust) to bind Rust code to ROS2 C++ code. It is already included in the `example_system` subdirectory of the code, so you shouldn't have to do anything extra to set this up. However, the inclusion of this bindings library makes our build process a little different than a regular Rust project (see *building and running* below). You won't be able to trivially run this with cargo.


### ROS2

*You can skip this step if you already have ROS2 working on your computer.*

First, try [these instructions](https://index.ros.org/doc/ros2/Installation/Foxy/Linux-Install-Binary/) for getting ROS2 on your system.

If it fails at downloading `ros-foxy-ros-base` because it couldn't be found, most likely ROS2 foxy is not available for your ARM architecture. You can compile ROS2 foxy yourself by following the instructions [here](https://index.ros.org/doc/ros2/Installation/Foxy/Linux-Development-Setup/).

If you get the following error when running `colcon build`:
```
undefined reference to `__atomic_exchange_8'
```
change the build command to be:
```
colcon build --symlink-install --cmake-args "-DCMAKE_SHARED_LINKER_FLAGS='-latomic'" "-DCMAKE_EXE_LINKER_FLAGS='-latomic'"
```


### Dataset
The visual odometry portion relies on an RGBD SLAM dataset. You can download the [TUM datasets here](https://vision.in.tum.de/data/datasets/rgbd-dataset/download) or use your own. Regardless of what dataset you choose, you'll need to provide an *associations file*. The TUM datasets do not provide their own associations file, so you will need to convert their dataset.

For TUM datasets, you can use [their script for associating color and depth images](https://vision.in.tum.de/data/datasets/rgbd-dataset/tools) to convert their dataset to an associations file. An example command for that script looks like:

``
python associate.py $DATASET/depth.txt $DATASET/rgb.txt > $DATASET/associations.txt
``

### Configuration file
You should include a file ``config.json`` in the ``example_system`` directory that looks similar to this:

``
{
    "freiberg_type": "fr1",
    "path_to_associations": "/full/path/to/dataset/associations.txt"
}
``

If you are using the TUM dataset, ``freiberg_type`` should be ``fr1``, ``fr2``, ``fr3``, or ``icl``. ``path_to_associations`` should be the *full* filepath to your associations file that you generated.

### Point to anodize in Cargo.toml
Change the path to anodize in `example_system/src/mc_system/mc_system/Cargo.toml` to point to the top-level `anodize` directory.

### Building, running, modifying

Because our example system is built on top of the ros2-rust repository, it unfortunately needs to use a non-Cargo-based build system. It uses colcon at the top level, which traverses the directory structure and runs each CMakeLists file. This process generates `build/`, `install/`, and `log/` directories in the `example_system` directory. We have provided a Makefile in the `example_system` directory to make the process of building and running easier.

(all the following commands should be run in `example_system`)
- To build: `make`
- To run: `make run`

If you want to make changes to our code, you will most likely be making those in the `example_system/src/mc_system/mc_system` directory ([see here](https://github.com/ssemenova/605/tree/master/example_system/src/mc_system/mc_system)). This code contains the meat of the example system; the rest of the files in the `example_system` directory are files needed for ros2-rust.

We have found that colcon and cmake are not the best at detecting *when* the Rust code needs to be rebuilt. You could delete the entire `build` and `install` directories, forcing a full rebuild, but this takes several minutes. Thus, the Makefile also provides several ways to minimally clean the build, so you can be sure that your changes are taking effect:

- After making a change to the Rust code inside `mc_system`, you should run: `make clean_rust_fast`
- After making a change to the CMakeLists file or Cargo.toml file in `mc_system`, you should run: `make clean_rust_slow`
- To do a full clean, you should run: `make clean_all`
