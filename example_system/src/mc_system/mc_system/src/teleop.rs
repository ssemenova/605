extern crate ncollide3d;

use device_query::{DeviceQuery, DeviceState, Keycode};

use Settings;
use std::convert::TryInto;

pub fn teleop_entrypoint(settings: Settings) {
	// Prepare node for publishing
    let context = rclrs::Context::default();
    let node = context.create_node("teleop_keyboard");
    let node = match node {
    	Ok(node) => node,
    	Err(_e) => return,
    };
    let publisher =
        node.create_publisher::<std_msgs::msg::SimpleTwist>("velocity_vector", rclrs::QOS_PROFILE_DEFAULT);  
    let publisher = match publisher {
    	Ok(publisher) => publisher,
    	Err(_e) => return,
    };
	
	// Read keyboard input
	let device_state = DeviceState::new();
	
	// Set defaults
	let max_lin_vel: f64 = settings.max_lin_vel.parse().unwrap();
	let max_ang_vel: f64 = settings.max_ang_vel.parse().unwrap();
	let lin_vel_step_size: f64 = settings.lin_vel_step_size.parse().unwrap();
	let ang_vel_step_size: f64 = settings.angle_vel_step_size.parse().unwrap();
	
	let mut target_linear_velocity = 0.0;
	let mut target_angular_velocity = 0.0;
	let mut control_linear_velocity = 0.0;
	let mut control_angular_velocity = 0.0;
	
    while context.ok() {
    	let mut linear = geometry_msgs::msg::Vector3::default();
    	let mut angular = geometry_msgs::msg::Vector3::default();
    	
    	let keys = device_state.get_keys();
    	
    	if keys.contains(&Keycode::W) {
			println!("Captured w");
    		target_linear_velocity = set_velocity(
    			target_linear_velocity + lin_vel_step_size,
    			max_lin_vel
    		)
    	} else if keys.contains(&Keycode::A) {
			println!("Captured a");
    		target_angular_velocity = set_velocity(
    			target_angular_velocity + ang_vel_step_size,
    			max_ang_vel
    		)
    	} else if keys.contains(&Keycode::D) {
			println!("Captured d");
    		target_angular_velocity = set_velocity(
    			target_angular_velocity - ang_vel_step_size,
    			max_ang_vel
    		)
    	} else if keys.contains(&Keycode::X) {
			println!("Captured x");
    		target_linear_velocity = set_velocity(
    			target_linear_velocity - lin_vel_step_size,
    			max_lin_vel
    		)
    	}
		
		println!("target linear velocity {}", target_linear_velocity);

        control_linear_velocity = make_simple_profile(
        	control_linear_velocity,
        	target_linear_velocity,
        	lin_vel_step_size / 2.0
    	);
        control_angular_velocity = make_simple_profile(
        	control_angular_velocity,
        	target_angular_velocity,
        	ang_vel_step_size / 2.0
    	);
		println!("control linear velocity {}", control_linear_velocity);

		linear.x = control_linear_velocity;
		angular.z = control_angular_velocity;
    	
    	let items = vec![linear.x, linear.y, linear.z, angular.x, angular.y, angular.z];
    	let vec_len = items.len();
    	
    	let message = std_msgs::msg::SimpleTwist {
			linear_x: linear.x,
			linear_y: linear.y,
			linear_z: linear.z,
			angular_x: angular.x,
			angular_y: angular.y,
			angular_z: angular.z
		};
    	
    	let result = publisher.publish(&message);
        match result {
        	Ok(result) => println!("published velocity {} {} {}", message.linear_x, message.linear_y, message.linear_z),
        	Err(result) => println!("Could not publish velocity"),
        };

        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}
    
fn set_velocity(velocity: f64, max_velocity: f64) -> f64 {
	if velocity < -max_velocity {
		return -max_velocity;
	} else if velocity > max_velocity {
		return max_velocity;
	} else {
		return velocity;
	}
}

fn make_simple_profile(output: f64, input: f64, slop: f64) -> f64 {
	if input > output {
		return if input < output + slop { input } else { output };
	} else if input < output {
		return if input > output - slop { input } else { output };
	} else {
		return input;
	}
}
