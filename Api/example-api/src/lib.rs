//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
// 
// This file is as an example, how to write an API in the Cube-OS framework
// 

// Dependencies
// #[cfg(feature = "ground")]
// extern crate ground;

use failure::{Fail};
use cubeos_service::*;
use serde::{Serialize,Deserialize};
use std::convert::From;
use cubeos_service::{Error,Result};
use strum_macros::{EnumString,Display,EnumIter};
use strum::IntoEnumIterator;

mod example;

// Make everything in example.rs public
pub use crate::example::*;

// Example Error type
// covers all Errors possible within your API, Service and Payload
#[derive(Debug, Fail, Clone, PartialEq)]
pub enum ExampleError {
    /// None
    #[fail(display = "None")]
    None,
    /// Example error
    #[fail(display = "Example error")]
    Err,
    /// Set Error
    #[fail(display = "Set error, only accepts ZERO or ONE")]
    SetErr,
    /// I2C Error
    #[fail(display = "I2C Error")]
    I2CError(std::io::ErrorKind),
    /// I2C Set Error
    #[fail(display = "I2C Set Error")]
    I2CSet,
    /// UART Error
    #[fail(display = "UART Error")]
    UARTError(uart_rs::UartError),
    /// UDP Error
    #[fail(display = "UDP Error")]
    UdpError(std::io::ErrorKind),
}
/// Implementation of Conversion of Example Error type 
/// to cubeos_service::Error (Error type that gets returned to GND)
/// 
/// cubeos-error::Error implements conversion for the following standard errors:
/// failure::Error -> cubeos_service::Error::Failure(String)
/// std::io::Error -> cubeos_service::Error::Io(u8)
/// Infallible -> cubeos_service::Error::Infallible
/// bincode::Error -> cubeos_service::Error::Bincode(u8)
/// PoisonError<MutexGuard<'a,T>> -> cubeos_service::Error::PoisonError
/// 
/// Any Errors in ExampleError must be converted to cubeos_service::Error::ServiceError(u8)
impl From<ExampleError> for Error {
    fn from(e: ExampleError) -> Error {
        log::error!("ExampleError: {:?}",e);
        match e {
            ExampleError::None => Error::ServiceError(0),
            ExampleError::Err => Error::ServiceError(1),
            ExampleError::SetErr => Error::ServiceError(2),
            ExampleError::I2CError(io) => Error::from(io),
            ExampleError::I2CSet => Error::ServiceError(3),
            ExampleError::UARTError(io) => Error::from(io),
            ExampleError::UdpError(io) => Error::from(io),
        }
    }
}
impl From<Error> for ExampleError {
    fn from(err: Error) -> ExampleError {
        match err {
            Error::ServiceError(0) => ExampleError::None,
            Error::ServiceError(1) => ExampleError::Err,
            Error::ServiceError(2) => ExampleError::SetErr,
            Error::ServiceError(3) => ExampleError::I2CSet,
            _ => ExampleError::Err, // or return a default error variant
        }
    }
}
impl From<uart_rs::UartError> for ExampleError {
    fn from(e: uart_rs::UartError) -> ExampleError {
        ExampleError::UARTError(e)
    }
}

// Example of Result Type used in the API
pub type ExampleResult<T> = core::result::Result<T,ExampleError>;

// Example of an Enum
// Enums can be used as Input (e.g. to choose a telemetry item) or 
// Output (e.g to show the state of a device (e.g. ON,OFF,IDLE,etc.))
#[derive(Debug,Default,Serialize,Deserialize,Copy,Clone,EnumIter,Display)]
// #[cfg_attr(feature = "ground", derive(Ground))]
pub enum ExampleEnum {
    #[default]
    Zero,
    One,
    All,
}

#[derive(Debug,Default,Serialize,Deserialize,Clone)]
pub struct TestStruct2 {
    t1: u64,
    te: ExampleEnum,
}

#[derive(Debug,Default,Serialize,Deserialize,Clone)]
pub struct TestStruct {
    in1: i16,
    in2: f64,
    struc: TestStruct2,
}

// Example of an Input/Output Struct
// It is necessary to also define a GraphQL equivalent for input structs
// (see example-service/graphql.rs)
#[derive(Debug,Serialize,Deserialize,Clone)]
// #[cfg_attr(feature = "ground", derive(Ground))]
pub struct ExampleInput {
    pub in_no: u16,
    pub in_no1: u32,
    pub in_no2: u16,
    pub in_str: String,
    pub in_bool: bool,
    pub in_struct: TestStruct,
}

#[derive(Debug,Serialize, Deserialize)]
// #[cfg_attr(feature = "ground", derive(Ground))]
pub struct ExampleOutput {
    pub out_no: Vec<u16>,
    pub out_str: String,
    pub out_bool: Vec<bool>,
}
// impl From<Vec<u8>> for ExampleOutput {
//     fn from(v: Vec<u8>) -> ExampleOutput {
//         ExampleOutput{
//             in_no: <u16>::from_be_bytes(&[v[0..2]]),
//             in_struct: Strukt(from(v[2..10].to_vec()))
//         }
//     }
// }