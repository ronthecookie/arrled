use crate::Controller;
use error::Error;
use rgb::*;

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
