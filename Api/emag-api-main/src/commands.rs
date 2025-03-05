use i2c_rs::Command;

pub mod set_charge_volt {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<u16, &'static str> {
        if data.len() == 2 {
            Ok((data[0] as u16) << 8 | (data[1] as u16))
        } else {
            Err("Invalid data length")
        }
    }

    pub fn command(perc_volt: u8) -> (Command, usize) {
        println!("Setting charge voltage to {}%", perc_volt);
        // println!("{}", format!("{:b}", perc_volt));
        let cmd = Command {
            cmd: 0x02,
            data: vec![perc_volt],
        };
        // println!("{}", format!("{:b}", cmd.2));
        let response_length = 2;
        (cmd, response_length)
    }
}

pub mod actuate {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(), &'static str> {
        if data == &[0x01] {
            Ok(())
        } else {
            Err("Invalid actuate response")
        }
    }

    pub fn command(axis: u8) -> (Command, usize) {
        let cmd = Command {
            cmd: 0x03,
            data: vec![axis],
        };
        let response_length = 1;
        (cmd, response_length)
    }
}

pub mod wipe {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(), &'static str> {
        if data == &[0x01] {
            Ok(())
        } else {
            Err("Invalid wipe response")
        }
    }

    pub fn command(axis: u8) -> (Command, usize) {
        let cmd = Command {
            cmd: 0x04,
            data: vec![axis],
        };
        let response_length = 1;
        (cmd, response_length)
    }
}
