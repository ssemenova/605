SHELL := /bin/bash

.PHONY: clean_rust_fast clean_rust_slow clean_all

install/mc_system/lib/mc_system/mc_system:
	source /opt/ros/foxy/setup.bash  
	colcon build
	
run: install/mc_system/lib/mc_system/mc_system
	ros2 run mc_system mc_system

clean_rust_fast:
	rm -rf build/mc_system/src
	rm -rf install/mc_system/lib/mc_system/mc_system
	
clean_rust_slow:
	rm -rf build/mc_system
	rm -rf install/mc_system/lib/mc_system/mc_system

clean_all:
	rm -rf build/ install/ log/
