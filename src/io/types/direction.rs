use core::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// Enum used to classify direction of data flow in relation to system.
///
/// Input objects generate data from the outside world;
/// output objects accept data, and manipulate the outside.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
pub enum IODirection {
    #[default]
    In,
    Out,
}

impl Display for IODirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IODirection::In => "Input",
            IODirection::Out => "Output",
        };
        write!(f, "{}", name)
    }
}
