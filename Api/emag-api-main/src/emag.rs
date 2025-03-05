use crate::commands::*;
use crate::objects::*;
use crate::EmagResult;
use i2c_rs::{Command, Connection as I2c};
use std::io;
use std::thread;
use std::time::Duration;

const INTER_COMMAND_DELAY: Duration = Duration::from_millis(60);

pub trait CuavaEmag {
    fn get_system_status(&mut self) -> EmagResult<Sys>;
    fn set_charge_volt(&self, volt: u8) -> EmagResult<u16>;
    fn actuate(&self, axis: Axis) -> EmagResult<()>;
    fn wipe(&self, axis: Axis) -> EmagResult<()>;
}

pub struct Emag {
    connection: I2c,
    sys_current: f32,
    x_hall: f32,
    y_hall: f32,
    z_hall: f32,
    cap_volt: f32,
}

impl Emag {
    pub fn new(path: &str, addr: u8) -> Self {
        Emag {
            connection: I2c::from_path(path, addr as u16),
            sys_current: 0.0,
            x_hall: 0.0,
            y_hall: 0.0,
            z_hall: 0.0,
            cap_volt: 0.0,
        }
    }
}

impl CuavaEmag for Emag {
    fn get_system_status(&mut self) -> EmagResult<Sys> {
        thread::sleep(INTER_COMMAND_DELAY);
        let status_request = Command {
            cmd: 0x01,
            data: vec![0x00],
        };

        let status_result: Result<Vec<u8>, io::Error> =
            self.connection
                .transfer(status_request, 20, Duration::from_millis(50));

        match status_result {
            Ok(count) => {
                let _current = (count[3] as u32) << 24
                    | (count[2] as u32) << 16
                    | (count[1] as u32) << 8
                    | (count[0] as u32);
                let _xhall = (count[7] as u32) << 24
                    | (count[6] as u32) << 16
                    | (count[5] as u32) << 8
                    | (count[4] as u32);
                let _yhall = (count[11] as u32) << 24
                    | (count[10] as u32) << 16
                    | (count[9] as u32) << 8
                    | (count[8] as u32);
                let _zhall = (count[15] as u32) << 24
                    | (count[14] as u32) << 16
                    | (count[13] as u32) << 8
                    | (count[12] as u32);
                let _capv = (count[19] as u32) << 24
                    | (count[18] as u32) << 16
                    | (count[17] as u32) << 8
                    | (count[16] as u32);

                self.sys_current = f32::from_bits(_current);
                self.x_hall = f32::from_bits(_xhall);
                self.y_hall = f32::from_bits(_yhall);
                self.z_hall = f32::from_bits(_zhall);
                self.cap_volt = f32::from_bits(_capv);

                let data = Sys {
                    sys_current: self.sys_current,
                    x_hall: self.x_hall,
                    y_hall: self.y_hall,
                    z_hall: self.z_hall,
                    cap_volt: self.cap_volt,
                };
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }

    fn set_charge_volt(&self, volt: u8) -> EmagResult<u16> {
        thread::sleep(INTER_COMMAND_DELAY);
        let command = set_charge_volt::command(volt);
        let response: Result<Vec<u8>, io::Error> =
            self.connection
                .transfer(command.0, command.1, Duration::from_millis(50));

        match response {
            Ok(count) => {
                if count.len() == 2 {
                    println!("count: {:?}", count);
                    let data = (count[0] as u16) << 8 | (count[1] as u16);
                    Ok(data)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid response",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    // xx xx 10 xx Z
    // xx xx 01 xx Y
    // xx xx 00 xx X
    // xx xx xx 00 Dir 0
    // xx xx xx 01 Dir 1
    // xx xx xx 10 Dir 2

    fn actuate(&self, axis: Axis) -> EmagResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        let command = actuate::command(axis.into());
        let response: Result<Vec<u8>, io::Error> =
            self.connection
                .transfer(command.0, command.1, Duration::from_millis(50));

        match response {
            Ok(count) => {
                if count.len() == 1 && count[0] == 0x01 {
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid response",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn wipe(&self, axis: Axis) -> EmagResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        let command = wipe::command(axis.into());
        let response: Result<Vec<u8>, io::Error> =
            self.connection
                .transfer(command.0, command.1, Duration::from_millis(50));

        match response {
            Ok(count) => {
                if count.len() == 1 && count[0] == 0x01 {
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid response",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
