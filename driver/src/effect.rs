use crate::{config::CPUTEMP_EFFECT_TARGET, Controller};
use error::Error;
use icmp;
use libmedium::{
    sensors::{Sensor, SensorBase},
    Hwmon, Hwmons,
};
use oref_red_alert::Alert;
use rgb::*;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub trait Effect {
    fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
    fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
    fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>>;
}

#[derive(Default)]
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

#[derive(Default)]
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

#[derive(Default)]
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
        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
    fn stop(&self, _controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Default)]
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

#[derive(Default)]
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
        thread::sleep(Duration::from_millis(200));

        Ok(())
    }
    fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;

        Ok(())
    }
}

#[derive(Default)]
pub struct Rainbow;
impl Effect for Rainbow {
    fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        thread::sleep(Duration::from_millis(500));
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
            thread::sleep(Duration::from_millis(150));
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

#[derive(Default)]
pub struct MissleAlert {
    area_count: Arc<AtomicU64>,
}

impl Effect for MissleAlert {
    fn init(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
        {
            let area_count = self.area_count.clone();
            thread::spawn(move || loop {
                let maybe_alert = Alert::get().unwrap();
                if let Some(alert) = maybe_alert {
                    area_count.store(alert.areas.len() as u64, Ordering::Relaxed)
                }
                thread::sleep(Duration::from_secs(2));
            })
            .join()
            .unwrap();
        }
        Ok(())
    }
    fn iter(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        let ac = self.area_count.load(Ordering::Relaxed);
        dbg!(ac);
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
        thread::sleep(Duration::from_millis(600 / ac.max(1)));
        if ac != 0 {
            controller.show_color(&RGB { r: 0, g: 0, b: 0 })?;
        }
        thread::sleep(Duration::from_millis(600 / ac.max(1)));
        Ok(())
    }
    fn stop(&self, controller: &mut Controller) -> Result<(), Box<dyn Error>> {
        controller.show_color(&RGB { r: 255, g: 0, b: 0 })?;
        Ok(())
    }
}
