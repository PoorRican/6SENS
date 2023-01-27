extern crate chrono;

mod container;
mod device;
mod io;
mod polling;
mod sensors;
mod settings;
mod units;

use chrono::Duration;

use crate::container::{Collection, Container, Containerized};
use crate::device::Sensor;
use crate::sensors::ph::MockPhSensor;
use crate::settings::Settings;
use crate::units::Ph;

fn main() {
    let _settings = Settings::initialize();

    let s0 = MockPhSensor::new("test name".to_string(), 0, Duration::seconds(5));
    let s1 = MockPhSensor::new("second sensor".to_string(), 1, Duration::seconds(10));
    let mut container: Container<Box<dyn Sensor<Ph>>, i32> = <dyn Sensor<Ph>>::container();
    container.add(0, Box::new(s0));
    container.add(1, Box::new(s1));
    dbg!(container._inner());
}
