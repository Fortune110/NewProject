//
// Copyright (C) 2022 CUAVA, The University of Sydney
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

//! CubeOS API for interacting with [ISIS ICEPSv2]
// Reference documentation: ISIS Electrical Power System 2 –Software ICD – IVID 7

// The API is contributed by Xueliang Bai <x.bai@sydney.edu.au>
// and Oscar Wilfred Thomas Ansted <oans4023@uni.sydney.edu.au> on behalf of the
// ARC Training Centre for CubeSats, UAVs & Their Applications (CUAVA) team (www.cuava.com.au)
// at the University of Sydney

// Dependancies
use i2c_rs::{Command, Connection as I2c};

use std::time::Duration;

use crate::error::*;
use crate::*;
use std::convert::From;

// // StID match shortcut
// fn match_st_id(typ: StID) -> u8 {
//     match typ {
//         StID::PduStid => PDU_STID,
//         StID::PbuStid => PBU_STID,
//         StID::PcuStid => PCU_STID,
//         StID::PiuStid => PIU_STID,
//         StID::OverrideStid => OVERRIDE_STID,
//     }
// }

pub struct Eps {
    pub i2c: I2c,
}

impl Eps {
    // Basic function to initialise an instance of the EpsStruct
    pub fn new(i2c_path: String, i2c_addr: u16) -> EpsResult<Self> {
        Ok(Self {
            i2c: I2c::from_path(&i2c_path, i2c_addr),
        })
    }

    // No-operation. Check system availability, without changing anything
    pub fn eps_ping(&self) -> EpsResult<()> {
        let cmd_code: u8 = NO_OP;
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data }; // i2c command

        let rx_len = 5;
        let delay = Duration::from_millis(50);

        // #[cfg(feature = "debug")]
        println! {"Eps Ping Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                // #[cfg(feature = "debug")]
                println! {"Eps Ping Response{:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Software reset. A reply to this command will not always be retrievable (system will shut down after this)
    pub fn sys_reset(&self, ret_key: u8) -> EpsResult<()> {
        // let ret_key: u8 = 0xA6; // Reset key
        let cmd_code: u8 = SYS_RESET; // command code
        let cmd: u8 = PIU_STID;

        // The value of ret_key needs to be set to 0xA6 for the command to be accepted.
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, ret_key].to_vec();
        let command = Command { cmd, data }; // i2c command

        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Reset Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Reset Response{:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Switches off any command-enable output bus channels.
    // All force-enable channels will remain enabled.
    pub fn shutdown_all(&self) -> EpsResult<()> {
        let cmd_code: u8 = CANCEL_OP;
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data }; // i2c command

        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Shutdown All Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Shutdown All Response{:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Resets the watchdog timer keeping the system from performing a reset (0x06)
    // Note tha any traffic with the system implicitly performs a watchdog reset.
    pub fn watchdog_reset(&self) -> EpsResult<()> {
        let cmd_code: u8 = WATCHDOG;
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data }; // i2c command

        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Watchdog Reset Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Watchdog Reset Response{:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn set_group(&self, typ_group: BusGroup, channels: BusChannelState) -> EpsResult<()> {
        // Match correct command arg
        let cmd_code: u8 = match typ_group {
            BusGroup::BusGroupOn => OUTPUT_BUS_GROUP_ON,
            BusGroup::BusGroupOff => OUTPUT_BUS_GROUP_OFF,
            BusGroup::BusGroupState => OUTPUT_BUS_GROUP_STATE,
        };

        let cmd: u8 = PIU_STID;
        let group_bytes = match typ_group {
            BusGroup::BusGroupOn => channels.on().to_le_bytes(),
            BusGroup::BusGroupOff => channels.off().to_le_bytes(),
            BusGroup::BusGroupState => {
                let current_state = match self.piu_hk(PIUHkSel::PIUEngHK) {
                    Ok(x) => x.stat_ch_on,
                    Err(e) => return Err(e),
                };
                match channels.state(current_state) {
                    Ok(x) => x.to_le_bytes(),
                    Err(e) => return Err(e),
                }
            }
        }; // use little endian for ISIS{

        // e.g. 0b1010011 (=0x0503, decimal 83). This switches output bus channels 0, 1, 4 and 6
        let data: Vec<u8> = [&[ALL_IVID, cmd_code, OVERRIDE_BID], &group_bytes[..]].concat();

        // let data:Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, group_bytes[0], group_bytes[1]].to_vec();
        let command = Command { cmd, data };
        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Set Group Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Set Group Response {:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Turn-on/off output bus channels with bitflag, leave unmarked unaltered. （0x10,0x12,0x14）
    // LSB bit corresponds to bus channel 0 (CH0),
    pub fn set_group_outputs(&self, typ_group: BusGroup, channels: Vec<u8>) -> EpsResult<()> {
        let bus_channels = BusChannelState::set(typ_group, channels)?;

        self.set_group(typ_group, bus_channels)
    }

    pub fn set_group_state(&self, typ_group: BusGroup, channels: BusChannelState) -> EpsResult<()> {
        self.set_group(typ_group, channels)
    }

    // Turn a single output bus channel on using the bus channel index. (0x16,0x18)
    // e.g. Index 0 represents channel 0 (CH0)
    pub fn set_single_output(&self, typ_channel: BusChannel, eps_ch_idx: u8) -> EpsResult<()> {
        // Check if rejection index error occurs within ISIS
        // Designed for ICEPSv2 (17 channels), Consider to remove this for larger iEPS modules
        if eps_ch_idx > 0x10 {
            return Err::<(), EpsError>(EpsError::InvalidInput);
        }

        let cmd_code: u8 = match typ_channel {
            BusChannel::On => OUTPUT_BUS_CHANNEL_ON,
            BusChannel::Off => OUTPUT_BUS_CHANNEL_OFF,
            BusChannel::Keep => return Err(EpsError::InvalidInput),
        };

        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, eps_ch_idx].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Set SingleOutput Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Set SingleOutput Response {:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    pub fn mode_switch(&self, mode: ModeSwitch) -> EpsResult<()> {
        let cmd_code: u8 = match mode {
            ModeSwitch::Nominal => SWITCH_TO_NOMINAL_MODE,
            ModeSwitch::Safety => SWITCH_TO_SAFETY_MODE,
        };

        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Mode Switch Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            // The (5th byte) responsed need to be parsed with match_stat
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Mode Switch Response {:?}",x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Get EPS System Status
    pub fn system_status(&self) -> EpsResult<SystemStatus> {
        let cmd_code: u8 = GET_SYS_STATUS;

        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 36;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Status Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Status Response {:?}", x};
                match match_stat(x[4]) {
                    Ok(()) => SystemStatus::try_from(x),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // 0x42  – Get Overcurrent Fault State
    pub fn overcurrent_state(&self) -> EpsResult<OverCurrentFaultState> {
        let cmd_code: u8 = GET_PDU_OC_FAULT_STATE;

        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 78;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"OverCurrent Status Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"OverCurrent Status Response {:?}", x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(OverCurrentFaultState::from(x)),
                    // Ok(()) => Ok(bincode::deserialize::<OverCurrentFaultState>(&x[6..50])?),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // // 0x44  – Get ABF Placed State
    // pub fn abf_state(&self) -> EpsResult<ABFState> {
    //     let cmd_code: u8 = GET_PBU_ABF_PLACED_STATE;

    //     let cmd: u8 = PIU_STID;
    //     let data:Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
    //     let command = Command{cmd, data};

    //     // Send command
    //     let rx_len = 8;
    //     let delay = Duration::from_millis(50);

    //     #[cfg(feature = "debug")]
    //     println!{"ABF State {:?}",command};

    //     match self.i2c.transfer(command, rx_len, delay) {
    //         Ok(x) => {
    //             #[cfg(feature = "debug")]
    //             println!{"ABF State Cmd {:?}", x};
    //             match match_stat(x[4]){
    //                 Ok(()) => Ok(ABFState::from(x)),
    //                 // Ok(()) => Ok(bincode::deserialize::<ABFState>(&x[6..8])?),
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         Err(_e) => Err(EpsError::TransferError),
    //     }

    // }

    // 0x52 and 0x54  – Get PDU Housekeeping Data (Engineering and Average Data)
    pub fn pdu_hk(&self, mode: PDUHkSel) -> EpsResult<PDUHk> {
        let cmd_code: u8 = match mode {
            PDUHkSel::PDURawHK => GET_PDU_HK_DATA_RAW,
            PDUHkSel::PDUEngHK => GET_PDU_HK_DATA_ENG,
            PDUHkSel::PDUAvgHK => GET_PDU_HK_DATA_AVRG,
        };
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 258;
        let delay = Duration::from_millis(50);

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => match match_stat(x[4]) {
                Ok(()) => Ok(PDUHk::from(x[6..156].to_vec())),
                Err(e) => Err(e),
            },
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // 0x62 and 0x64  – Get PBU Housekeeping Data (Engineering and Average Data)
    pub fn pbu_hk(&self, mode: PBUHkSel) -> EpsResult<PBUHk> {
        let cmd_code: u8 = match mode {
            PBUHkSel::PBURawHK => GET_PBU_HK_DATA_RAW,
            PBUHkSel::PBUEngHK => GET_PBU_HK_DATA_ENG,
            PBUHkSel::PBUAvgHK => GET_PBU_HK_DATA_AVRG,
        };
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 84;
        let delay = Duration::from_millis(50);

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => match match_stat(x[4]) {
                Ok(()) => Ok(PBUHk::from(x[6..34].to_vec())),
                Err(e) => Err(e),
            },
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // 0x72 and 0x74  – Get PCU Housekeeping Data (Engineering and Average Data)
    pub fn pcu_hk(&self, mode: PCUHkSel) -> EpsResult<PCUHk> {
        let cmd_code: u8 = match mode {
            PCUHkSel::PCURawHK => GET_PCU_HK_DATA_RAW,
            PCUHkSel::PCUEngHK => GET_PCU_HK_DATA_ENG,
            PCUHkSel::PCUAvgHK => GET_PCU_HK_DATA_AVRG,
        };
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 72;
        let delay = Duration::from_millis(50);

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => match match_stat(x[4]) {
                Ok(()) => Ok(PCUHk::from(x[6..].to_vec())),
                Err(e) => Err(e),
            },
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // 0xA2 and 0xA4  – Get PIU Housekeeping Data (Engineering and Average Data)
    pub fn piu_hk(&self, mode: PIUHkSel) -> EpsResult<PIUHk> {
        let cmd_code: u8 = match mode {
            PIUHkSel::PIURawHK => GET_PIU_HK_DATA_RAW,
            PIUHkSel::PIUEngHK => GET_PIU_HK_DATA_ENG,
            PIUHkSel::PIUAvgHK => GET_PIU_HK_DATA_AVRG,
        };
        let cmd: u8 = PIU_STID;
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID].to_vec();
        let command = Command { cmd, data };

        // Send command
        // 116 bytes w/o daughterboard, 274 bytes with daughterboard
        let rx_len = 274;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"PIU HK Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"PIU HK Response {:?}", x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(PIUHk::from(x)),
                    // One reseved byte. Starting from the 6th byte
                    // Ok(()) => Ok(bincode::deserialize::<PIUHk>(&x[6..184])?),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    // Correct the unit’s unix time with the specified amount of seconds.
    // unix time value is returned as part of the “0x40 (0x41) – Get System Status” response,
    pub fn correct_time(&self, time_correction: i32) -> EpsResult<()> {
        let _cmd_code: u8 = CORRECT_TIME;
        let cmd: u8 = PIU_STID;

        let mut data: Vec<u8> = [ALL_IVID, 0xC4, OVERRIDE_BID].to_vec();
        data.append(&mut time_correction.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let rx_len = 1;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Correct Time Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Correct Time Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    //  Write all reset cause counters to zero in persistent memory (0xC6)
    pub fn reset_all_counters(&self) -> EpsResult<()> {
        let cmd_code: u8 = RST_CAUSE_CNTR;
        let cmd: u8 = PIU_STID;
        let zero_key: u8 = 0xA7;

        // Zero key: 0xA7. Any other value causes this command to be rejected with a parameter error
        // XL: Not sure why zero_key is defined as i32 in manual, to be tested
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, zero_key].to_vec();
        let command = Command { cmd, data }; // i2c command

        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Reset All Counters Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Reset All Counters Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }
}
