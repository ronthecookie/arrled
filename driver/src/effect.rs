use crate::Controller;
use error::Error;
use icmp;
use rgb::*;
use std::{
	net::{IpAddr, Ipv4Addr},
	time::Duration,
};

pub trait Effect {
	fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
	fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
	fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
}

pub struct JustRed;
impl Effect for JustRed {
	fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
		Ok(())
	}
	fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
	fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}

pub struct MovingRedDot;
impl Effect for MovingRedDot {
	fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
	fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		for i in 0..(controller.led_count - 1) {
			let before = if i == 0 {
				controller.led_count - 2
			} else {
				i - 1
			};
			controller.set_led(&before, &RGB { r: 0, g: 0, b: 0 })?;
			controller.set_led(&i, &RGB { r: 255, g: 0, b: 0 })?;
		}
		Ok(())
	}
	fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}

pub struct PingIP;
impl Effect for PingIP {
	fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
		Ok(())
	}
	fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 131));
		let ping = icmp::IcmpSocket::connect(ip);
		let mut ping = ping.unwrap();
		let payload: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let result = ping.send(payload);
		println!("result: {:?}", result);
		if result.is_ok() {
			controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
		} else {
			controller.show_color(&RGB {
				r: 255,
				g: 0,
				b: 255,
			})?;
		}
		std::thread::sleep(Duration::from_millis(500));
		Ok(())
	}
	fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
