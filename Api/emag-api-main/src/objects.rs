// use serde::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumIter};

#[derive(Default, Serialize, Deserialize)]
pub struct Sys {
    pub sys_current: f32,
    pub x_hall: f32,
    pub y_hall: f32,
    pub z_hall: f32,
    pub cap_volt: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, EnumIter, Clone, Display, PartialEq)]
pub enum Axis {
    #[default]
    X_plus,
    Y_plus,
    Z_plus,
    X_minus,
    Y_minus,
    Z_minus,
}
impl From<Axis> for u8 {
    fn from(axis: Axis) -> u8 {
        match axis {
            Axis::Z_plus => 0b10_00,
            Axis::Y_plus => 0b01_00,
            Axis::X_plus => 0b00_00,
            Axis::Z_minus => 0b10_01,
            Axis::Y_minus => 0b01_01,
            Axis::X_minus => 0b00_01,
        }
    }
}
impl FromStr for Axis {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X_plus" => Ok(Axis::X_plus),
            "Y_plus" => Ok(Axis::Y_plus),
            "Z_plus" => Ok(Axis::Z_plus),
            "X_minus" => Ok(Axis::X_minus),
            "Y_minus" => Ok(Axis::Y_minus),
            "Z_minus" => Ok(Axis::Z_minus),
            _ => Err(()),
        }
    }
}
