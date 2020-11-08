use crate::{config::CPUTEMP_EFFECT_TARGET, Controller};
use error::Error;
use icmp;
use libmedium::{
    sensors::{Sensor, SensorBase},
    Hwmon, Hwmons,
};
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
    fn iter(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn stop(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct MovingRedDot;
impl Effect for MovingRedDot {
    fn init(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
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
    fn stop(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
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
    fn stop(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct ArrGeeBee;
impl Effect for ArrGeeBee {
    fn init(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
        controller.show_color(&RGB { r: 0, g: 255, b: 0 })?;
        controller.show_color(&RGB { r: 0, g: 0, b: 255 })?;

        Ok(())
    }
    fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;

        Ok(())
    }
}

pub struct CPUTemp;
impl Effect for CPUTemp {
    fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        let hwmons = Hwmons::parse_read_only()?;

        let hwmon = hwmons
            .into_iter()
            .filter(|x| x.2.name() == CPUTEMP_EFFECT_TARGET)
            .next()
            .unwrap()
            .2;

        let temp = hwmon.temp(1).unwrap().read_input()?.as_degrees_celsius();
        println!("temp={:.3}", temp);

        controller.show_color(&RGB {
            r: temp as u8,
            g: 0,
            b: 0,
        })?;

        // don't need to update that much
        std::thread::sleep(Duration::from_millis(200));

        Ok(())
    }
    fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;

        Ok(())
    }
}

pub struct Rainbow;
impl Effect for Rainbow {
    fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        std::thread::sleep(Duration::from_millis(500));
        let frequency = 0.7;
        for i in 0..(controller.led_count - 1) {
            controller.set_led(
                &i,
                &RGB {
                    r: ((frequency * (i + 5) as f64).sin() * 127.0 + 127.0) as u8,
                    g: ((frequency * (i + 5) as f64 + 2.0).sin() * 127.0 + 127.0) as u8,
                    b: ((frequency * (i + 5) as f64 + 4.0).sin() * 127.0 + 127.0) as u8,
                },
            )?;
            std::thread::sleep(Duration::from_millis(150));
        }
        Ok(())
    }
    fn iter(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn stop(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
