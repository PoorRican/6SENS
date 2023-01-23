use std::convert::From;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ph(pub f32);

impl Ph {
    /// Abstract pH by constraining float values to 0.0 to 14.0
    ///
    /// # Arguments
    ///
    /// * `val`: a float between 0.0 and 14.0. Returns an error string if value is out of bounds.
    ///
    /// returns: Ph
    pub fn new(value: f32) -> Result<Self, String> {
        if value < 0.0 || value > 14.0 {
            return Err(format!("Invalid pH value: {}", value));
        }
        Ok(Ph(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for Ph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}", self.value())
    }
}

impl From<f32> for Ph {
    fn from(value: f32) -> Self {
        Ph::new(value).unwrap()
    }
}