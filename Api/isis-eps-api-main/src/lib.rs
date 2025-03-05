// #![deny(missing_docs)]

pub use crate::config::*;
pub use crate::eps::*;
pub use crate::error::*;
pub use crate::objects::*;

mod config;
mod eps;
mod error;
mod objects;

// ID's
// const PDU_STID: u8 = 0x11;
// const PBU_STID: u8 = 0x12;
// const PCU_STID: u8 = 0x13;
const PIU_STID: u8 = 0x1A;
// const OVERRIDE_STID: u8 = 0x00;
const ALL_IVID: u8 = 0x07;
// const OVERRIDE_IVID: u8 = 0x00;
// const PDU_BID: u8 = 0x00;
// const PBU_BID: u8 = 0x00;
// const PCU_BID: u8 = 0x00;
const OVERRIDE_BID: u8 = 0x00;

// System Operational command
const SYS_RESET: u8 = 0xAA;
const NO_OP: u8 = 0x02;
const CANCEL_OP: u8 = 0x04;
const WATCHDOG: u8 = 0x06;
// const CORRECT_TIME: u8 = 0x08;
const CORRECT_TIME: u8 = 0xC4;
const RST_CAUSE_CNTR: u8 = 0xC6;

// Bus Group Operational Command
const OUTPUT_BUS_GROUP_ON: u8 = 0x10;
const OUTPUT_BUS_GROUP_OFF: u8 = 0x12;
const OUTPUT_BUS_GROUP_STATE: u8 = 0x14;
const OUTPUT_BUS_CHANNEL_ON: u8 = 0x16;
const OUTPUT_BUS_CHANNEL_OFF: u8 = 0x18;

// Mode switch command
const SWITCH_TO_NOMINAL_MODE: u8 = 0x30;
const SWITCH_TO_SAFETY_MODE: u8 = 0x32;

// Data request commands
const GET_SYS_STATUS: u8 = 0x40;
const GET_PDU_OC_FAULT_STATE: u8 = 0x42;
// const GET_PBU_ABF_PLACED_STATE: u8 = 0x44;
const GET_PDU_HK_DATA_RAW: u8 = 0x50;
const GET_PDU_HK_DATA_ENG: u8 = 0x52;
const GET_PDU_HK_DATA_AVRG: u8 = 0x54;
const GET_PBU_HK_DATA_RAW: u8 = 0x60;
const GET_PBU_HK_DATA_ENG: u8 = 0x62;
const GET_PBU_HK_DATA_AVRG: u8 = 0x64;
const GET_PCU_HK_DATA_RAW: u8 = 0x70;
const GET_PCU_HK_DATA_ENG: u8 = 0x72;
const GET_PCU_HK_DATA_AVRG: u8 = 0x74;

// Config commands
const GET_CONFIG_PARA: u8 = 0x82;
const SET_CONFIG_PARA: u8 = 0x84;
const RESET_CONFIG_PARA: u8 = 0x86;
const RESET_CONFIG_ALL: u8 = 0x90;
const LOAD_CONFIG: u8 = 0x92;
const SAVE_CONFIG: u8 = 0x94;

// Data request commands
const GET_PIU_HK_DATA_RAW: u8 = 0xA0;
const GET_PIU_HK_DATA_ENG: u8 = 0xA2;
const GET_PIU_HK_DATA_AVRG: u8 = 0xA4;

// Most other functions return the STAT parameter. Write function here to check the the STAT for the error code
fn match_stat(typ: u8) -> EpsResult<()> {
    // is it <T, Error> ?
    match typ {
        0x00 => Ok(()),
        0x80 => Ok(()),
        0x01 => Err(EpsError::Rejected),
        0x02 => Err(EpsError::InvalidCommandCode),
        0x03 => Err(EpsError::ParameterMissing),
        0x04 => Err(EpsError::Parameterinvalid),
        0x05 => Err(EpsError::UnavailableMode),
        0x06 => Err(EpsError::InvalidSystemType),
        _ => Err(EpsError::InternalProcessing),
        // Reserved values: 0x10, 0x20, 0x40
        // NEW 0x80 set when the response is read for the first time
    }
}
