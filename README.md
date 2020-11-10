# Anodize: Language-Level Guarantees for Mixed Criticality Systems

## Running example mixed criticality system

The example mixed criticality system is run from the *mcsystem crate* (in the `system` subdirectory). This program contains the *primordial thread* (found in `system/src/main.rs`) that configures the system and spawns each "program" in the mixed criticality system. The programs are:

1. A visual odometry program that reads a SLAM dataset and outputs the location of the robot at each time step. This code is written in *pure Rust* and is, for the most part, unchanged code from [this github repository](https://github.com/mpizenberg/visual-odometry-rs). It can be found in `system/src/visodom`.

While the example mixed criticality system largely lives in the `system` subdirectory of this repository, it has two crate dependencies: *anodize* (within the `anodize` subdirectory of this repository) and *ros2_rust* (from [this github repository](https://github.com/ros2-rust/ros2_rust)). Thus, the easiest way to get the system running is to download this entire repository and follow the steps below to configure each part of the system.

There are several components of the example mixed criticality system that need to be configured:

1. Getting ROS2 running on your computer
2. Configuring ROS2_rust so ROS2 can be called from our Rust code
3. Downloading an RGBD SLAM dataset and converting it into an associations file
4. Modifying `mcsystem`'s configuration file to point to the dataset

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


### ROS2 <-> Rust bindings

We use [ros2_rust](https://github.com/ros2-rust/ros2_rust) to bind Rust code to ROS2 C++ code.



### Dataset
The visual odometry portion relies on an RGBD SLAM dataset. You can download the [TUM datasets here](https://vision.in.tum.de/data/datasets/rgbd-dataset/download) or use your own. Regardless of what dataset you choose, you'll need to provide an *associations file*. The TUM datasets do not provide their own associations file, so you will need to convert their dataset.

For TUM datasets, you can use [their script for associating color and depth images](https://vision.in.tum.de/data/datasets/rgbd-dataset/tools) to convert their dataset to an associations file. An example command for that script looks like:

``
python associate.py $DATASET/depth.txt $DATASET/rgb.txt > $DATASET/associations.txt
``

### Configuration file
You should include a file ``config.json`` located in the ``system`` directory that looks similar to this:

``
{
    "freiberg_type": "fr1",
    "path_to_associations": "/full/path/to/dataset/associations.txt"
}
``

If you are using the TUM dataset, ``freiberg_type`` should be ``fr1``, ``fr2``, ``fr3``, or ``icl``. ``path_to_associations`` should be the *full* filepath to your associations file that you generated.


### Running
```
cd system/
cargo run --release
```
