extern crate error;
extern crate rand;
extern crate rgb;
extern crate serial;

pub mod effect;
mod magic;
mod config;

use std::env;
use std::time::Duration;

use effect::*;
use error::Error;
use rgb::*;
use serial::{SerialPort, SystemPort};
use std::io;
use std::io::prelude::*;

fn main() {
	for arg in env::args_os().skip(1) {
		let port = serial::open(&arg).unwrap();
		interact(port).unwrap();
	}
}

fn interact(mut port: SystemPort) -> Result<(), Box<dyn Error>> {
	&mut port.reconfigure(&|settings| {
		settings.set_baud_rate(serial::Baud9600)?;
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityNone);
		settings.set_stop_bits(serial::Stop1);
		settings.set_flow_control(serial::FlowNone);
		Ok(())
	})?;

	&mut port.set_timeout(Duration::from_millis(1000))?;

	let mut controller = Controller {
		port: port,
		led_count: 60,
	};
	controller.clear_leds()?;

	let eff = config::CURRENT_EFFECT;
	
	eff.init(&mut controller)?;
	loop {
		eff.iter(&mut controller)?;
	}

	Ok(())
}

pub struct Controller {
	port: SystemPort,
	led_count: u8,
}
impl Controller {
	pub fn set_led(&mut self, index: &u8, rgb: &RGB<u8>) -> io::Result<()> {
		let buf: Vec<u8> = vec![magic::PREFIX, magic::SET_LED, *index, rgb.r, rgb.g, rgb.b];
		self.port.write(&buf)?;
		self.port.flush()?;

		Ok(())
	}

	fn clear_leds(&mut self) -> io::Result<()> {
		for i in 0..(self.led_count - 1) {
			self.set_led(&i, &RGB { r: 0, g: 0, b: 0 })?;
		}
		Ok(())
	}

	pub fn show_color(&mut self, rgb: &RGB<u8>) -> io::Result<()> {
		let buf: Vec<u8> = vec![magic::PREFIX, magic::SHOW_ALL, rgb.r, rgb.g, rgb.b];
		self.port.write(&buf)?;
		self.port.flush()?;

		Ok(())
	}
}
