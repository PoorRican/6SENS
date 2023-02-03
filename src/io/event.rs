use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::io::{Device, IOData, LogType};
use crate::storage::{Container, Containerized};

/// Encapsulates `IOData` alongside of timestamp and device data
#[derive(Debug, Serialize, Deserialize)]
pub struct IOEvent {
    pub version_id: i32,
    pub sensor_id: i32,
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub data: IOData,
}

// TODO: add kind to `IOEvent`
impl IOEvent {
    /// Generate sensor event.
    ///
    /// # Arguments
    ///
    /// * `device`: struct that has implemented the `Device` trait
    /// * `timestamp`: timestamp of event
    /// * `value`: value to include in
    ///
    /// returns: SensorEvent
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn create(
        device: &(impl Device + ?Sized),
        timestamp: DateTime<Utc>,
        value: f64,
    ) -> Self {
        let info = device.get_metadata();
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = IOData {
            kind: info.kind.clone(),
            data: value,
        };
        IOEvent {
            version_id,
            sensor_id,
            timestamp,
            data,
        }
    }
}

/// Return a new instance of `Container` with for storing `IOEvent` which are accessed by `DateTime<Utc>` as keys
impl Containerized<IOEvent, DateTime<Utc>> for IOEvent
where
{
    fn container() -> LogType {
        Container::<IOEvent, DateTime<Utc>>::new()
    }
}
