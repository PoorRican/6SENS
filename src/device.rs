/// Provide Low-level Device Functionality

use chrono::{Duration, Utc};

use crate::io;


pub trait Device<T> {
    fn get_info(&self) -> &DeviceInfo<T>;
    fn name(&self) -> String;
    fn id(&self) -> i32;
}


pub trait Sensor<T>: Device<T> {
    fn read(&self) -> T;

    fn get_event(&self) -> io::IOEvent<T> where Self: Sized {
        io::IOEvent::create(self,
                          Utc::now(),
                          self.read())
    }
}



/// Defines an interface for an input device that needs to be calibrated
pub trait Calibrated {
    /// Initiate the calibration procedures for a specific device instance.
    fn calibrate(&self) -> bool;
}


#[derive(Debug, Clone)]
/// Encapsulates individual device info
/// Meant to used as a struct attribute via `new()`
pub struct DeviceInfo<T> {
    pub name: String,
    pub version_id: i32,
    pub sensor_id: i32,
    pub kind: io::IOKind,

    min_value: T,   // min value (in SI units)
    max_value: T,   // max value (in SI units)
    resolution: T,  // resolution of sensor (in SI units)

    min_delay: Duration, // minimum delay between sensing events
}


impl<T> DeviceInfo<T> {
    pub fn new(name: String, version_id: i32, sensor_id: i32,
                  kind: io::IOKind, min_value: T, max_value: T, resolution: T, min_delay: Duration) -> DeviceInfo<T> {
        DeviceInfo {
            name, version_id, sensor_id,
            kind, min_value, max_value, resolution, min_delay
        }
    }
}

