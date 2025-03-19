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
// Comments generated in parts with GPT-3 (see disclaimer in README)

use serde::*;

// Dependencies for UART and I2C
use i2c_rs::{Command, Connection as I2c};
use uart_rs::{Connection as Uart};
use udp_rs::{Connection as Udp};
use std::cell::RefCell;
use std::time::Duration;
use std::thread;
// #[cfg(feature = "ground")]
// use ground::*;
use super::*;

const I2C_GET: u8 = 0x01;
const I2C_SET: u8 = 0x10;
const UART_GET: u8 = 0x02;
const UART_SET: u8 = 0x20;

// Example of Struct containing the functions to connect to the payload
// #[derive(Serialize,Deserialize)]
pub struct ExampleStruct {
    // I2C connection
    i2c_connection: I2c,
    
    // UART connection
    uart_connection: Uart,
    // Buffer needed for UART connections
    buffer: RefCell<Vec<u8>>,

    // UDP connection
    udp_connection: Udp,

    // SPI connection 
    // for later use
    // spi_connection: Spi,

    // example-values
    ex_no0: u16,
    ex_no1: u16,
    ex_str: String,
    ex_bool0: bool,
    ex_bool1: bool,
}
impl ExampleStruct {
    /// Basic function to initialise an instance of the ExampleStruct
    ///
    /// # Arguments
    ///
    /// * `i2c_path` - A string that represents the path to the i2c connection
    /// * `i2c_addr` - A u16 that represents the address of the i2c connection
    /// * `uart_path` - A string that represents the path to the uart connection
    /// * `uart_setting` - A serial::PortSettings that represents the settings of the uart connection
    /// * `uart_timeout` - A Duration that represents the timeout of the uart connection
    /// * `udp_path` - A string that represents the path to the udp connection
    /// * `udp_to` - A string that represents the address of the udp connection
    ///
    /// # Output
    ///
    /// * `ExampleResult<Self>` - Returns `Self` or Error of type ExampleError
    ///
    pub fn new(
        i2c_path: String,
        i2c_addr: u16,
        uart_path: String,
        uart_setting: serial::PortSettings,
        uart_timeout: Duration,
        // API's Listener Address
        udp_path: String,
        // Payload Address
        udp_to: String,
    ) -> ExampleResult<Self> {
        Ok(Self{
            i2c_connection: I2c::from_path(&i2c_path,i2c_addr),
            uart_connection: Uart::from_path(&uart_path,uart_setting,uart_timeout),
            buffer: RefCell::new(Vec::new()),
            udp_connection: Udp::from_path(udp_path, udp_to),
            // spi_connection: Spi::from_path(spi),
            
            ex_no0: 0u16,
            ex_no1: 0u16,
            ex_str: "".to_string(),
            ex_bool0: false,
            ex_bool1: false,
        })
    }

    // examples of get and set functions that use the previously defined
    // Enum and Structs as In-/Output

    /// This function is used to get values from the struct.
    ///
    /// # Arguments
    ///
    /// * `g` - An enum that specifies which values to get.
    ///
    /// # Output
    ///
    /// * `ExampleOutput` - A struct containing the requested values.
    pub fn get_values(&mut self, g: ExampleEnum) -> ExampleResult<ExampleOutput> {
        match g {
            ExampleEnum::Zero => Ok(ExampleOutput{
                out_no: vec![self.ex_no0],
                out_str: self.ex_str.to_string(),
                out_bool: vec![self.ex_bool0],
            }),
            ExampleEnum::One => Ok(ExampleOutput{
                out_no: vec![self.ex_no1],
                out_str: self.ex_str.to_string(),
                out_bool: vec![self.ex_bool1],
            }),
            ExampleEnum::All => self.get_all_values()
        }
    }

    fn get_all_values(&self) -> ExampleResult<ExampleOutput> {
        Ok(ExampleOutput{
            out_no: vec![self.ex_no0,self.ex_no1],
            out_str: self.ex_str.to_string(),
            out_bool: vec![self.ex_bool0,self.ex_bool1],
        })
    }

    /// This function sets the values of the struct.
    ///
    /// # Arguments
    ///
    /// * `s` - An instance of ExampleInput
    /// * `e` - An instance of ExampleEnum
    ///
    /// # Output
    ///
    /// * `()` - If successful
    /// * `ExampleError::SetErr` - If unsuccessful
    ///
    pub fn set_values(&mut self, s: ExampleInput, e: ExampleEnum) -> ExampleResult<()> {
        match e {
            ExampleEnum::Zero => {
                self.ex_no0 = s.in_no;
                self.ex_str = s.in_str.to_string();
                self.ex_bool0 = s.in_bool;
                Ok(())
            },
            ExampleEnum::One => {
                self.ex_no1 = s.in_no;
                self.ex_str = s.in_str.to_string();
                self.ex_bool1 = s.in_bool;
                Ok(())
            },
            _ => {
                Err(ExampleError::SetErr)
            },
        }   
    }

    /// I2C Example Transfer (Write-Read)
    /// The I2C transfer function is the preferred function used for commanding I2C payloads.
    /// It has the structure: 
    /// connection.transfer(&self, command: Command, rx_len: usize, delay: Duration)
    /// 
    /// Examples for Write and Read are given below for completeness

    /// This function performs a write and read of payload data via I2C
    ///
    /// # Arguments
    ///
    /// * `self` - the struct that contains the i2c connection
    ///
    /// # Output
    ///
    /// * `ExampleResult<Vec<u8>>` - the result of the i2c transfer
    ///
    /// # Errors
    ///
    /// * `ExampleError::I2CError` - if the i2c transfer fails
    ///
    pub fn get_i2c(&mut self) -> ExampleResult<Vec<u8>> {
        let cmd: u8 = I2C_GET;
        let rx_len = 1;
        let delay = Duration::from_millis(50);

        let data: Vec<u8> = Vec::new();
        let command = Command{cmd, data};

        match self.i2c_connection.transfer(command, rx_len, delay) {
            Ok(x) => Ok(x),
            Err(e) => Err(ExampleError::I2CError(e.kind())),
        }
    }

    /// This function performs a write with input data and read of payload data via I2C
    ///
    /// # Arguments
    ///
    /// * `i` - The data to be sent to the payload.
    ///
    /// # Output
    ///
    /// * `ExampleResult<()>` - The result of the operation.
    ///
    /// # Errors
    ///
    /// * `ExampleError::I2CError` - if the i2c transfer fails
    ///
    pub fn set_i2c(&mut self, i: u8) -> ExampleResult<()> {
        let cmd: u8 = I2C_SET;
        let rx_len = 1;
        let delay = Duration::from_millis(50);

        let mut data: Vec<u8> = Vec::new();
        data.push(i);
        let command = Command{cmd, data};

        match self.i2c_connection.transfer(command, rx_len, delay) {
            Ok(x) => {
                if x[0] == cmd {
                    Ok(())
                } else {
                    Err(ExampleError::I2CSet)
                } 
            },               
            Err(e) => Err(ExampleError::I2CError(e.kind())),
        }
    }

    // This function serves as an example how to write a payload via I2C
    //
    // The I2C write function has the structure:
    // write(&self, command: Command)
    // 
    // pub fn i2c_write(&self, i: ExampleInput) -> ExampleResult<()> {
    //     let cmd: u8 = i.in_no as u8;

    //     if cmd != 0 {
    //         let data: Vec<u8> = Vec::new();
    //         data.push(i.in_str.to_vec());
    //         let command = Command{cmd, data};

    //         match self.connection.write(command) {
    //             Ok(()) => Ok(()),
    //             Err(_) => Err(ExampleError::Err),
    //         }
    //     }
    // }
    
    // I2C Example Read
    // This function serves as an example how to read from a payload via I2C
    //
    // The I2C read function has the structure:
    // read(&self, command: Command, rx_len: usize)
    // 
    // pub fn i2c_read(&self, cmd: Command) -> ExampleResult<ExampleOutput> {       
    //     let rx_len: usize = 10;
    //     match self.connection.read(cmd.cmd, rx_len) {
    //         Ok(x) => Ok(ExampleOutput{
    //                 out_no: x as u8,
    //                 out_str: "".to_string(),
    //                 out_bool: true,
    //             }),
    //         Err(_) => Err(ExampleError::Err),
    //     }                
    // }

    
    /// UART Examples
    /// These functions serves as an example how to communicate with a payload via UART
    /// 
    /// The UART read function has the structure:
    /// read(&self, len: usize, timeout: Duration)
    /// 
    /// The UART write function has the structure:
    /// write(&self, data: &[u8])
    /// 

    /// Returns a vector of bytes from the UART connection.
    ///
    /// # Arguments
    ///
    /// * `self` - the `Example` struct
    ///
    /// # Output
    ///
    /// * `ExampleResult<Vec<u8>>` - a vector of bytes
    /// 
    pub fn get_uart(&mut self) -> ExampleResult<Vec<u8>> {
        let get = &[UART_GET];

        match self.uart_connection.write(get) {
            Ok(_) => {
                // sleep 10ms
                thread::sleep(Duration::from_millis(10));

                // Reads 1 byte, with a timeout of 1ms
                Ok(self.uart_connection.read(1, Duration::from_millis(1))?)
            }
            Err(e) => Err(ExampleError::UARTError(e)),
        }
    }

    /// This example explores the possibilty of the payload not returning anything
    /// If the UART payload sends a reply to your set-command, implementation is similar to the get_uart command
    /// Writes data to the UART
    ///
    /// # Arguments
    ///
    /// * `input` - The input to set the UART to.
    ///
    /// # Output
    ///
    /// * `ExampleResult<()>` - The result of the operation.
    /// 
    pub fn set_uart(&mut self, input: u8) -> ExampleResult<()> {
        let set = &[UART_SET, input];

        match self.uart_connection.write(set) {
            Ok(_) => Ok(()),
            Err(e) => Err(ExampleError::UARTError(e)),      
        }
    }

    /// UDP Examples
    /// This is an example for a simple get function via UDP
    ///
    /// # Arguments
    ///
    /// * `command` - A vector of bytes to send to the device
    /// * `rx_len` - The number of bytes to receive from the device
    ///
    /// # Output
    ///
    /// * `ExampleResult<Vec<u8>>` - a vector of bytes
    /// 
    pub fn get_udp(&self, command: Vec<u8>) -> ExampleResult<Vec<u8>> {
        match self.udp_connection.transfer(command) {
            Ok(v) => Ok(v),
            Err(e) => Err(ExampleError::UdpError(e.kind())),
        }
    }

    // This example explores the possibilty of the payload not returning anything
    // It is however recommended to expect a reply from the payload and use the transfer function
    pub fn set_udp(&self, command: Vec<u8>) -> ExampleResult<()> {
        match self.udp_connection.write(command) {
            Ok(()) => Ok(()),
            Err(e) => Err(ExampleError::UdpError(e.kind())),
        }
    }
}