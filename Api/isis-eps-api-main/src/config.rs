use crate::error::*;

use crate::eps::*;
use crate::ConfigParamRead::*;
use crate::ConfigParamWrite::*;
use crate::*;
use i2c_rs::Command;
use serde::*;
use std::time::Duration;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, EnumIter, Display, Hash)]
pub enum Output {
    U32(u32),
    U16(u16),
    I16(i16),
    U8(u8),
    I8(i8),
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWriteU32 {
    #[default]
    ChStartupEnaBf,
    ChStartupKey,
    ChLatchoffEnaBf,
    ChLatchoffKey,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWriteU16 {
    #[default]
    TtcWdgTimeout,
    TtcWdgTimeoutKey,
    ChStartupDelay(u8),
    ChLatchoffDelay(u8),
    SafetyVoltLoThr,
    SafetyVoltHiThr,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWriteI16 {
    #[default]
    LoThrBp1Heater,
    // LoThrBp2Heater,
    // LoThrBp3Heater,
    HiThrBp1Heater,
    // HiThrBp2Heater,
    // HiThrBp3Heater,
    LoThrBp1Unbal,
    // LoThrBp2Unbal,
    // LoThrBp3Unbal,
    HiThrBp1Unbal,
    // HiThrBp2Unbal,
    // HiThrBp3Unbal,
    McuTempBias,
    McuTempPremul,
    McuTempPosDiv,
    Bp1Temp1Bias,
    Bp1Temp2Bias,
    Bp1Temp3Bias,
    // Bp2Temp1Bias,
    // Bp2Temp2Bias,
    // Bp2Temp3Bias,
    // Bp3Temp1Bias,
    // Bp3Temp2Bias,
    // Bp3Temp3Bias,
    Bp1Temp1Premul,
    Bp1Temp2Premul,
    Bp1Temp3Premul,
    // Bp2Temp1Premul,
    // Bp2Temp2Premul,
    // Bp2Temp3Premul,
    // Bp3Temp1Premul,
    // Bp3Temp2Premul,
    // Bp3Temp3Premul,
    Bp1Temp1PosDiv,
    Bp1Temp2PosDiv,
    Bp1Temp3PosDiv,
    // Bp2Temp1PosDiv,
    // Bp2Temp2PosDiv,
    // Bp2Temp3PosDiv,
    // Bp3Temp1PosDiv,
    // Bp3Temp2PosDiv,
    // Bp3Temp3PosDiv,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWriteU8 {
    #[default]
    BoardId,
    BoardIdKey,
    RavgStrengthP2,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWriteI8 {
    #[default]
    AutoHeatEnaBP1,
    // AutoHeatEnaBP2,
    // AutoHeatEnaBP3,
    AutoBalEnaBP1,
    // AutoBalEnaBP2,
    // AutoBalEnaBP3,
    Vd1AlwaysEna,
    Vd2AlwaysEna,
    Vd3AlwaysEna,
    Vd4AlwaysEna,
    Vd5AlwaysEna,
    Vd6AlwaysEna,
    Vd1AlwaysDisa,
    Vd2AlwaysDisa,
    Vd3AlwaysDisa,
    Vd4AlwaysDisa,
    Vd5AlwaysDisa,
    Vd6AlwaysDisa,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamWrite {
    ChStartupEnaBf,
    ChStartupKey,
    ChLatchoffEnaBf,
    ChLatchoffKey,
    TtcWdgTimeout,
    TtcWdgTimeoutKey,
    ChStartupDelay(u8),
    ChLatchoffDelay(u8),
    SafetyVoltLoThr,
    SafetyVoltHiThr,
    LoThrBp1Heater,
    HiThrBp1Heater,
    // LoThrBp2Heater,
    // HiThrBp2Heater,
    // LoThrBp3Heater,
    // HiThrBp3Heater,
    LoThrBp1Unbal,
    HiThrBp1Unbal,
    // LoThrBp2Unbal,
    // HiThrBp2Unbal,
    // LoThrBp3Unbal,
    // HiThrBp3Unbal,
    McuTempBias,
    McuTempPremul,
    McuTempPosDiv,
    Bp1Temp1Bias,
    Bp1Temp2Bias,
    Bp1Temp3Bias,
    // Bp2Temp1Bias,
    // Bp2Temp2Bias,
    // Bp2Temp3Bias,
    // Bp3Temp1Bias,
    // Bp3Temp2Bias,
    // Bp3Temp3Bias,
    Bp1Temp1Premul,
    Bp1Temp2Premul,
    Bp1Temp3Premul,
    // Bp2Temp1Premul,
    // Bp2Temp2Premul,
    // Bp2Temp3Premul,
    // Bp3Temp1Premul,
    // Bp3Temp2Premul,
    // Bp3Temp3Premul,
    Bp1Temp1PosDiv,
    Bp1Temp2PosDiv,
    Bp1Temp3PosDiv,
    // Bp2Temp1PosDiv,
    // Bp2Temp2PosDiv,
    // Bp2Temp3PosDiv,
    // Bp3Temp1PosDiv,
    // Bp3Temp2PosDiv,
    // Bp3Temp3PosDiv,
    #[default]
    BoardId,
    BoardIdKey,
    RavgStrengthP2,
    AutoHeatEnaBP1,
    // AutoHeatEnaBP2,
    // AutoHeatEnaBP3,
    AutoBalEnaBP1,
    // AutoBalEnaBP2,
    // AutoBalEnaBP3,
    Vd1AlwaysEna,
    Vd2AlwaysEna,
    Vd3AlwaysEna,
    Vd4AlwaysEna,
    Vd5AlwaysEna,
    Vd6AlwaysEna,
    Vd1AlwaysDisa,
    Vd2AlwaysDisa,
    Vd3AlwaysDisa,
    Vd4AlwaysDisa,
    Vd5AlwaysDisa,
    Vd6AlwaysDisa,
}
impl ConfigParamWrite {
    pub fn get_id(&self) -> u16 {
        match self {
            ChStartupEnaBf => 0x6002,
            ChStartupKey => 0x6003,
            ChLatchoffEnaBf => 0x6004,
            ChLatchoffKey => 0x6005,
            TtcWdgTimeout => 0x4000,
            TtcWdgTimeoutKey => 0x4001,
            ChStartupDelay(0) => 0x4002,
            ChStartupDelay(1) => 0x4003,
            ChStartupDelay(2) => 0x4004,
            ChStartupDelay(3) => 0x4005,
            ChStartupDelay(4) => 0x4006,
            ChStartupDelay(5) => 0x4007,
            ChStartupDelay(6) => 0x4008,
            ChStartupDelay(7) => 0x4009,
            ChStartupDelay(8) => 0x400A,
            ChStartupDelay(9) => 0x400B,
            ChStartupDelay(10) => 0x400C,
            ChStartupDelay(11) => 0x400D,
            ChStartupDelay(12) => 0x400E,
            ChStartupDelay(13) => 0x400F,
            ChStartupDelay(14) => 0x4010,
            ChStartupDelay(15) => 0x4011,
            ChStartupDelay(16) => 0x4012,
            ChStartupDelay(17) => 0x4013,
            ChStartupDelay(18) => 0x4014,
            ChStartupDelay(19) => 0x4015,
            ChStartupDelay(20) => 0x4016,
            ChStartupDelay(21) => 0x4017,
            ChStartupDelay(22) => 0x4018,
            ChStartupDelay(23) => 0x4019,
            ChStartupDelay(24) => 0x401A,
            ChStartupDelay(25) => 0x401B,
            ChStartupDelay(26) => 0x401C,
            ChStartupDelay(27) => 0x401D,
            ChStartupDelay(28) => 0x401E,
            ChStartupDelay(29) => 0x401F,
            ChStartupDelay(30) => 0x4020,
            ChStartupDelay(31) => 0x4021,
            ChLatchoffDelay(0) => 0x4022,
            ChLatchoffDelay(1) => 0x4023,
            ChLatchoffDelay(2) => 0x4024,
            ChLatchoffDelay(3) => 0x4025,
            ChLatchoffDelay(4) => 0x4026,
            ChLatchoffDelay(5) => 0x4027,
            ChLatchoffDelay(6) => 0x4028,
            ChLatchoffDelay(7) => 0x4029,
            ChLatchoffDelay(8) => 0x402A,
            ChLatchoffDelay(9) => 0x402B,
            ChLatchoffDelay(10) => 0x402C,
            ChLatchoffDelay(11) => 0x402D,
            ChLatchoffDelay(12) => 0x402E,
            ChLatchoffDelay(13) => 0x402F,
            ChLatchoffDelay(14) => 0x4030,
            ChLatchoffDelay(15) => 0x4031,
            ChLatchoffDelay(16) => 0x4032,
            ChLatchoffDelay(17) => 0x4033,
            ChLatchoffDelay(18) => 0x4034,
            ChLatchoffDelay(19) => 0x4035,
            ChLatchoffDelay(20) => 0x4036,
            ChLatchoffDelay(21) => 0x4037,
            ChLatchoffDelay(22) => 0x4038,
            ChLatchoffDelay(23) => 0x4039,
            ChLatchoffDelay(24) => 0x403A,
            ChLatchoffDelay(25) => 0x403B,
            ChLatchoffDelay(26) => 0x403C,
            ChLatchoffDelay(27) => 0x403D,
            ChLatchoffDelay(28) => 0x403E,
            ChLatchoffDelay(29) => 0x403F,
            ChLatchoffDelay(30) => 0x4040,
            ChLatchoffDelay(31) => 0x4041,
            SafetyVoltLoThr => 0x4042,
            SafetyVoltHiThr => 0x4043,
            LoThrBp1Heater => 0x3000,
            HiThrBp1Heater => 0x3003,
            LoThrBp1Unbal => 0x3006,
            HiThrBp1Unbal => 0x3009,
            McuTempBias => 0x300C,
            McuTempPremul => 0x300D,
            McuTempPosDiv => 0x300E,
            Bp1Temp1Bias => 0x300F,
            Bp1Temp2Bias => 0x3010,
            Bp1Temp3Bias => 0x3011,
            Bp1Temp1Premul => 0x3018,
            Bp1Temp2Premul => 0x3019,
            Bp1Temp3Premul => 0x301A,
            Bp1Temp1PosDiv => 0x3021,
            Bp1Temp2PosDiv => 0x3022,
            Bp1Temp3PosDiv => 0x3023,
            BoardId => 0x2000,
            BoardIdKey => 0x2001,
            RavgStrengthP2 => 0x2002,
            AutoHeatEnaBP1 => 0x1001,
            AutoBalEnaBP1 => 0x1004,
            Vd1AlwaysEna => 0x1007,
            Vd2AlwaysEna => 0x1008,
            Vd3AlwaysEna => 0x1009,
            Vd4AlwaysEna => 0x100A,
            Vd5AlwaysEna => 0x100B,
            Vd6AlwaysEna => 0x100C,
            Vd1AlwaysDisa => 0x100E,
            Vd2AlwaysDisa => 0x100F,
            Vd3AlwaysDisa => 0x1010,
            Vd4AlwaysDisa => 0x1011,
            Vd5AlwaysDisa => 0x1012,
            Vd6AlwaysDisa => 0x1013,
            ChStartupDelay(_) => 0x4000,
            ChLatchoffDelay(_) => 0x4000,
        }
    }
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            0x1001 => Some(AutoHeatEnaBP1),
            0x1004 => Some(AutoBalEnaBP1),
            0x1007 => Some(Vd1AlwaysEna),
            0x1008 => Some(Vd2AlwaysEna),
            0x1009 => Some(Vd3AlwaysEna),
            0x100A => Some(Vd4AlwaysEna),
            0x100B => Some(Vd5AlwaysEna),
            0x100C => Some(Vd6AlwaysEna),
            0x100E => Some(Vd1AlwaysDisa),
            0x100F => Some(Vd2AlwaysDisa),
            0x1010 => Some(Vd3AlwaysDisa),
            0x1011 => Some(Vd4AlwaysDisa),
            0x1012 => Some(Vd5AlwaysDisa),
            0x1013 => Some(Vd6AlwaysDisa),
            0x2000 => Some(BoardId),
            0x2001 => Some(BoardIdKey),
            0x2002 => Some(RavgStrengthP2),
            0x3000 => Some(LoThrBp1Heater),
            0x3003 => Some(HiThrBp1Heater),
            0x3006 => Some(LoThrBp1Unbal),
            0x3009 => Some(HiThrBp1Unbal),
            0x300C => Some(McuTempBias),
            0x300D => Some(McuTempPremul),
            0x300E => Some(McuTempPosDiv),
            0x300F => Some(Bp1Temp1Bias),
            0x3010 => Some(Bp1Temp2Bias),
            0x3011 => Some(Bp1Temp3Bias),
            0x3018 => Some(Bp1Temp1Premul),
            0x3019 => Some(Bp1Temp2Premul),
            0x301A => Some(Bp1Temp3Premul),
            0x3021 => Some(Bp1Temp1PosDiv),
            0x3022 => Some(Bp1Temp2PosDiv),
            0x3023 => Some(Bp1Temp3PosDiv),
            0x4000 => Some(TtcWdgTimeout),
            0x4001 => Some(TtcWdgTimeoutKey),
            0x4000..=0x401F => Some(ChStartupDelay(id.to_le_bytes()[0] - 0x00)),
            0x4022..=0x403F => Some(ChLatchoffDelay(id.to_le_bytes()[0] - 0x22)),
            0x4042 => Some(SafetyVoltLoThr),
            0x4043 => Some(SafetyVoltHiThr),
            0x6002 => Some(ChStartupEnaBf),
            0x6003 => Some(ChStartupKey),
            0x6004 => Some(ChLatchoffEnaBf),
            0x6005 => Some(ChLatchoffKey),
            _ => None,
        }
    }
    pub fn get_len(&self) -> usize {
        match self.get_id() {
            0x6000..=0x6FFF => 4,
            0x3000..=0x4FFF => 2,
            0x1000..=0x2FFF => 1,
            _ => 0, // Return 0 for unknown codes
        }
    }
    pub fn iter_id() -> impl Iterator<Item = u16> {
        (0x0000..=0xFFFF).filter(|&id| ConfigParamWrite::from_id(id).is_some())
    }
}

impl From<ConfigParamWriteU32> for ConfigParamWrite {
    fn from(cu32: ConfigParamWriteU32) -> ConfigParamWrite {
        match cu32 {
            ConfigParamWriteU32::ChStartupEnaBf => ConfigParamWrite::ChStartupEnaBf,
            ConfigParamWriteU32::ChStartupKey => ConfigParamWrite::ChStartupKey,
            ConfigParamWriteU32::ChLatchoffEnaBf => ConfigParamWrite::ChLatchoffEnaBf,
            ConfigParamWriteU32::ChLatchoffKey => ConfigParamWrite::ChLatchoffKey,
        }
    }
}

impl From<ConfigParamWriteU16> for ConfigParamWrite {
    fn from(cu16: ConfigParamWriteU16) -> ConfigParamWrite {
        match cu16 {
            ConfigParamWriteU16::TtcWdgTimeout => ConfigParamWrite::TtcWdgTimeout,
            ConfigParamWriteU16::TtcWdgTimeoutKey => ConfigParamWrite::TtcWdgTimeoutKey,
            ConfigParamWriteU16::ChStartupDelay(delay) => ConfigParamWrite::ChStartupDelay(delay),
            ConfigParamWriteU16::ChLatchoffDelay(delay) => ConfigParamWrite::ChLatchoffDelay(delay),
            ConfigParamWriteU16::SafetyVoltLoThr => ConfigParamWrite::SafetyVoltLoThr,
            ConfigParamWriteU16::SafetyVoltHiThr => ConfigParamWrite::SafetyVoltHiThr,
        }
    }
}

impl From<ConfigParamWriteI16> for ConfigParamWrite {
    fn from(ci16: ConfigParamWriteI16) -> ConfigParamWrite {
        match ci16 {
            ConfigParamWriteI16::LoThrBp1Heater => ConfigParamWrite::LoThrBp1Heater,
            ConfigParamWriteI16::HiThrBp1Heater => ConfigParamWrite::HiThrBp1Heater,
            ConfigParamWriteI16::LoThrBp1Unbal => ConfigParamWrite::LoThrBp1Unbal,
            ConfigParamWriteI16::HiThrBp1Unbal => ConfigParamWrite::HiThrBp1Unbal,
            ConfigParamWriteI16::McuTempBias => ConfigParamWrite::McuTempBias,
            ConfigParamWriteI16::McuTempPremul => ConfigParamWrite::McuTempPremul,
            ConfigParamWriteI16::McuTempPosDiv => ConfigParamWrite::McuTempPosDiv,
            ConfigParamWriteI16::Bp1Temp1Bias => ConfigParamWrite::Bp1Temp1Bias,
            ConfigParamWriteI16::Bp1Temp2Bias => ConfigParamWrite::Bp1Temp2Bias,
            ConfigParamWriteI16::Bp1Temp3Bias => ConfigParamWrite::Bp1Temp3Bias,
            ConfigParamWriteI16::Bp1Temp1Premul => ConfigParamWrite::Bp1Temp1Premul,
            ConfigParamWriteI16::Bp1Temp2Premul => ConfigParamWrite::Bp1Temp2Premul,
            ConfigParamWriteI16::Bp1Temp3Premul => ConfigParamWrite::Bp1Temp3Premul,
            ConfigParamWriteI16::Bp1Temp1PosDiv => ConfigParamWrite::Bp1Temp1PosDiv,
            ConfigParamWriteI16::Bp1Temp2PosDiv => ConfigParamWrite::Bp1Temp2PosDiv,
            ConfigParamWriteI16::Bp1Temp3PosDiv => ConfigParamWrite::Bp1Temp3PosDiv,
        }
    }
}

impl From<ConfigParamWriteU8> for ConfigParamWrite {
    fn from(cu8: ConfigParamWriteU8) -> ConfigParamWrite {
        match cu8 {
            ConfigParamWriteU8::BoardId => ConfigParamWrite::BoardId,
            ConfigParamWriteU8::BoardIdKey => ConfigParamWrite::BoardIdKey,
            ConfigParamWriteU8::RavgStrengthP2 => ConfigParamWrite::RavgStrengthP2,
        }
    }
}

impl From<ConfigParamWriteI8> for ConfigParamWrite {
    fn from(ci8: ConfigParamWriteI8) -> ConfigParamWrite {
        match ci8 {
            ConfigParamWriteI8::AutoHeatEnaBP1 => ConfigParamWrite::AutoHeatEnaBP1,
            ConfigParamWriteI8::AutoBalEnaBP1 => ConfigParamWrite::AutoBalEnaBP1,
            ConfigParamWriteI8::Vd1AlwaysEna => ConfigParamWrite::Vd1AlwaysEna,
            ConfigParamWriteI8::Vd2AlwaysEna => ConfigParamWrite::Vd2AlwaysEna,
            ConfigParamWriteI8::Vd3AlwaysEna => ConfigParamWrite::Vd3AlwaysEna,
            ConfigParamWriteI8::Vd4AlwaysEna => ConfigParamWrite::Vd4AlwaysEna,
            ConfigParamWriteI8::Vd5AlwaysEna => ConfigParamWrite::Vd5AlwaysEna,
            ConfigParamWriteI8::Vd6AlwaysEna => ConfigParamWrite::Vd6AlwaysEna,
            ConfigParamWriteI8::Vd1AlwaysDisa => ConfigParamWrite::Vd1AlwaysDisa,
            ConfigParamWriteI8::Vd2AlwaysDisa => ConfigParamWrite::Vd2AlwaysDisa,
            ConfigParamWriteI8::Vd3AlwaysDisa => ConfigParamWrite::Vd3AlwaysDisa,
            ConfigParamWriteI8::Vd4AlwaysDisa => ConfigParamWrite::Vd4AlwaysDisa,
            ConfigParamWriteI8::Vd5AlwaysDisa => ConfigParamWrite::Vd5AlwaysDisa,
            ConfigParamWriteI8::Vd6AlwaysDisa => ConfigParamWrite::Vd6AlwaysDisa,
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
    Hash,
)]
pub enum ConfigParamRead {
    ChForceEnaUseBf,
    ChStartUpEnaUseBf,
    ChLatchoffEnaUseBf,
    Vd1AllocChBf,
    Vd2AllocChBf,
    Vd3AllocChBf,
    Vd4AllocChBf,
    Vd5AllocChBf,
    Vd6AllocChBf,
    SwciChCmdEnaBf,
    SwciChCmdDisaBf,
    TtcI2cSlaveAddr,
    ConfNvmSaveCntr,
    ConfNvmSaveChks,
    RstCause,
    RstCntrPwron,
    RstCntrWdg,
    RstCntrCmd,
    RstCntrMcu,
    RstCntrEmlopo,
    RstCntrMcuRaw,
    EmlopoVoltLoThr,
    EmlopoVoltHiThr,
    EmlopoPeriod,
    SafetyVoltLoThrUsed,
    SafetyVoltHiThrUsed,
    SafetyLinger,
    TtcWdgTimeoutUsed,
    TtcPevCmdElapsed,
    AdcMcuTempV25T30,
    AdcMcuTempV25T85,
    #[default]
    Stid,
    Ivid,
    BidUsed,
    BootResumeShort,
    ConfParamChanged,
}
impl ConfigParamRead {
    pub fn get_id(&self) -> u16 {
        match self {
            ChForceEnaUseBf => 0x6809,
            ChStartUpEnaUseBf => 0x680A,
            ChLatchoffEnaUseBf => 0x680B,
            Vd1AllocChBf => 0x680C,
            Vd2AllocChBf => 0x680D,
            Vd3AllocChBf => 0x680E,
            Vd4AllocChBf => 0x680F,
            Vd5AllocChBf => 0x6810,
            Vd6AllocChBf => 0x6811,
            SwciChCmdEnaBf => 0x6813,
            SwciChCmdDisaBf => 0x6814,
            TtcI2cSlaveAddr => 0x4800,
            ConfNvmSaveCntr => 0x4801,
            ConfNvmSaveChks => 0x4802,
            RstCause => 0x4803,
            RstCntrPwron => 0x4804,
            RstCntrWdg => 0x4805,
            RstCntrCmd => 0x4806,
            RstCntrMcu => 0x4807,
            RstCntrEmlopo => 0x4808,
            RstCntrMcuRaw => 0x4809,
            EmlopoVoltLoThr => 0x480A,
            EmlopoVoltHiThr => 0x480B,
            EmlopoPeriod => 0x480C,
            SafetyVoltLoThrUsed => 0x480D,
            SafetyVoltHiThrUsed => 0x480E,
            SafetyLinger => 0x480F,
            TtcWdgTimeoutUsed => 0x4810,
            TtcPevCmdElapsed => 0x4811,
            AdcMcuTempV25T30 => 0x3800,
            AdcMcuTempV25T85 => 0x3801,
            Stid => 0x2800,
            Ivid => 0x2801,
            BidUsed => 0x2802,
            BootResumeShort => 0x2803,
            ConfParamChanged => 0x1800,
        }
    }
    pub fn get_len(&self) -> usize {
        match self.get_id() {
            0x6800..=0x6FFF => 4,
            0x4800..=0x48FF => 2,
            0x3800..=0x38FF => 2,
            0x2800..=0x28FF => 1,
            0x1800..=0x18FF => 1,
            _ => 0, // Return 0 for unknown codes
        }
    }
}

pub trait EpsConfig {
    fn get_config_para_write(&self, param: ConfigParamWrite) -> EpsResult<Output>;
    fn get_config_para_read(&self, param: ConfigParamRead) -> EpsResult<Output>;
    fn set_config_para_u32(&self, param: ConfigParamWriteU32, input: u32) -> EpsResult<Output>;
    fn set_config_para_u16(&self, param: ConfigParamWriteU16, input: u16) -> EpsResult<Output>;
    fn set_config_para_i16(&self, param: ConfigParamWriteI16, input: i16) -> EpsResult<Output>;
    fn set_config_para_u8(&self, param: ConfigParamWriteU8, input: u8) -> EpsResult<Output>;
    fn set_config_para_i8(&self, param: ConfigParamWriteI8, input: i8) -> EpsResult<Output>;
    fn reset_param(&self, param: ConfigParamWrite) -> EpsResult<Output>;
    fn reset_all_conf(&self) -> EpsResult<()>;
    fn load_config(&self) -> EpsResult<()>;
    fn save_config_force(&self) -> EpsResult<()>;
    fn save_config(&self) -> EpsResult<()>;
    fn calculate_checksum(&self) -> EpsResult<u16>;
    fn get_config_data(&self) -> EpsResult<Vec<u8>>;
}
impl EpsConfig for Eps {
    fn get_config_para_write(&self, param: ConfigParamWrite) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = param.get_id().to_le_bytes();
        let data: Vec<u8> = [ALL_IVID, GET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};

        match param.get_id() {
            0x6000..=0x60FF => {
                let rx_len = 12;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => {
                                Ok(Output::U32(u32::from_le_bytes([x[8], x[9], x[10], x[11]])))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x4000..=0x40FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U16(u16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x3000..=0x30FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I16(i16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x2000..=0x20FF => {
                let rx_len = 9;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U8(u8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x1000..=0x10FF => {
                let rx_len = 9;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I8(i8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            _ => Err(EpsError::InvalidInput),
        }
    }

    fn get_config_para_read(&self, param: ConfigParamRead) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = param.get_id().to_le_bytes();
        let data: Vec<u8> = [ALL_IVID, GET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};

        match param.get_id() {
            0x6800..=0x68FF => {
                let rx_len = 12;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => {
                                Ok(Output::U32(u32::from_le_bytes([x[8], x[9], x[10], x[11]])))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x4800..=0x48FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U16(u16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x3800..=0x38FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I16(i16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x2800..=0x28FF => {
                let rx_len = 8;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U8(u8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x1800..=0x18FF => {
                let rx_len = 8;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I8(i8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            _ => Err(EpsError::InvalidInput),
        }
    }

    fn set_config_para_u32(&self, param: ConfigParamWriteU32, input: u32) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = ConfigParamWrite::from(param).get_id().to_le_bytes();
        let mut data: Vec<u8> = [ALL_IVID, SET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        data.append(&mut input.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};
        let rx_len = 12;
        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Config Response {:?}",x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(Output::U32(u32::from_le_bytes([x[8], x[9], x[10], x[11]]))),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn set_config_para_u16(&self, param: ConfigParamWriteU16, input: u16) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = ConfigParamWrite::from(param).get_id().to_le_bytes();
        let mut data: Vec<u8> = [ALL_IVID, SET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        data.append(&mut input.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};
        let rx_len = 10;
        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Config Response {:?}",x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(Output::U16(u16::from_le_bytes([x[8], x[9]]))),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn set_config_para_i16(&self, param: ConfigParamWriteI16, input: i16) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = ConfigParamWrite::from(param).get_id().to_le_bytes();
        let mut data: Vec<u8> = [ALL_IVID, SET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        data.append(&mut input.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};
        let rx_len = 10;
        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Config Response {:?}",x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(Output::I16(i16::from_le_bytes([x[8], x[9]]))),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn set_config_para_u8(&self, param: ConfigParamWriteU8, input: u8) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = ConfigParamWrite::from(param).get_id().to_le_bytes();
        let mut data: Vec<u8> = [ALL_IVID, SET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        data.append(&mut input.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};
        let rx_len = 9;
        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Config Response {:?}",x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(Output::U8(u8::from_le_bytes([x[8]]))),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn set_config_para_i8(&self, param: ConfigParamWriteI8, input: i8) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = ConfigParamWrite::from(param).get_id().to_le_bytes();
        let mut data: Vec<u8> = [ALL_IVID, SET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        data.append(&mut input.to_le_bytes().to_vec());

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};
        let rx_len = 9;
        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"System Config Response {:?}",x};
                match match_stat(x[4]) {
                    Ok(()) => Ok(Output::I8(i8::from_le_bytes([x[8]]))),
                    Err(e) => Err(e),
                }
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn reset_param(&self, param: ConfigParamWrite) -> EpsResult<Output> {
        let cmd: u8 = PIU_STID;

        let id = param.get_id().to_le_bytes();
        let data: Vec<u8> = [ALL_IVID, RESET_CONFIG_PARA, OVERRIDE_BID, id[0], id[1]].to_vec();

        let command = Command { cmd, data };

        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"System Config Cmd{:?}",command};

        match param.get_id() {
            0x6000..=0x60FF => {
                let rx_len = 12;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => {
                                Ok(Output::U32(u32::from_le_bytes([x[8], x[9], x[10], x[11]])))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x4000..=0x40FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U16(u16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x3000..=0x30FF => {
                let rx_len = 10;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I16(i16::from_le_bytes([x[8], x[9]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x2000..=0x20FF => {
                let rx_len = 8;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::U8(u8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            0x1000..=0x10FF => {
                let rx_len = 8;
                match self.i2c.transfer(command, rx_len, delay) {
                    Ok(x) => {
                        #[cfg(feature = "debug")]
                        println! {"System Config Response {:?}",x};
                        match match_stat(x[4]) {
                            Ok(()) => Ok(Output::I8(i8::from_le_bytes([x[8]]))),
                            Err(e) => Err(e),
                        }
                    }
                    Err(_e) => Err(EpsError::TransferError),
                }
            }
            _ => Err(EpsError::InvalidInput),
        }
    }

    fn reset_all_conf(&self) -> EpsResult<()> {
        let cmd_code: u8 = RESET_CONFIG_ALL;
        let config_key: u8 = 0xA7;

        let cmd: u8 = PIU_STID;
        // Config key must be 0xA7, any other value will be rejected with a parameter error
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, config_key].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Reset All Config Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Reset All Config Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn load_config(&self) -> EpsResult<()> {
        let cmd_code: u8 = LOAD_CONFIG;
        let config_key: u8 = 0xA7;

        let cmd: u8 = PIU_STID;
        // Config key must be 0xA7, any other value will be rejected with a parameter error
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, config_key].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Load Config Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Load Config Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn save_config_force(&self) -> EpsResult<()> {
        let cmd_code: u8 = SAVE_CONFIG;
        let config_key: u8 = 0xA7;
        let checksum = [0x00, 0x00];

        let cmd: u8 = PIU_STID;
        // Config key must be 0xA7, any other value will be rejected with a parameter error
        let data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, config_key, checksum[0], checksum[1]].to_vec();
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Save Config Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Save Config Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn save_config(&self) -> EpsResult<()> {
        let cmd_code: u8 = SAVE_CONFIG;
        let config_key: u8 = 0xA7;
        let checksum = match self.calculate_checksum() {
            Ok(x) => x.to_le_bytes(),
            Err(e) => return Err(e),
        };

        let cmd: u8 = PIU_STID;
        // Config key must be 0xA7, any other value will be rejected with a parameter error
        let mut data: Vec<u8> = [ALL_IVID, cmd_code, OVERRIDE_BID, config_key].to_vec();
        data.append(&mut checksum.to_vec());
        let command = Command { cmd, data };

        // Send command
        let rx_len = 5;
        let delay = Duration::from_millis(50);

        #[cfg(feature = "debug")]
        println! {"Save Config Cmd {:?}",command};

        match self.i2c.transfer(command, rx_len, delay) {
            Ok(x) => {
                #[cfg(feature = "debug")]
                println! {"Save Config Response {:?}", x};
                match_stat(x[4])
            }
            Err(_e) => Err(EpsError::TransferError),
        }
    }

    fn calculate_checksum(&self) -> EpsResult<u16> {
        let mut crc: u16 = 0xFFFF;

        let config_data = match self.get_config_data() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        for byte in config_data.iter() {
            crc ^= u16::from(*byte) << 8;
            for _ in 0..8 {
                if crc & 0x8000 != 0 {
                    crc = (crc << 1) ^ 0x1021;
                } else {
                    crc <<= 1;
                }
            }
        }

        Ok(crc)
    }

    fn get_config_data(&self) -> EpsResult<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();

        for param in ConfigParamWrite::iter_id() {
            let param_data =
                match self.get_config_para_write(ConfigParamWrite::from_id(param).unwrap()) {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
            match param_data {
                Output::U32(x) => result.append(&mut x.to_le_bytes().to_vec()),
                Output::U16(x) => result.append(&mut x.to_le_bytes().to_vec()),
                Output::I16(x) => result.append(&mut x.to_le_bytes().to_vec()),
                Output::U8(x) => result.append(&mut x.to_le_bytes().to_vec()),
                Output::I8(x) => result.append(&mut x.to_le_bytes().to_vec()),
            }
        }

        Ok(result)
    }
}
