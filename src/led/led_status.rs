use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LedStatus {
    ON,
    OFF,
}

impl ToString for LedStatus {
    fn to_string(&self) -> String {
        match self {
            LedStatus::ON => String::from("ON"),
            LedStatus::OFF => String::from("OFF"),
        }
    }
}
