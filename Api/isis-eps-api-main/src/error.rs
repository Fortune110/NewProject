use cubeos_service::Error;
use failure::Fail;

// Error list
#[derive(Debug, Fail, Clone, PartialEq)]
pub enum EpsError {
    /// Example error
    #[fail(display = "Eps Error")]
    Err,
    /// I2C Error
    #[fail(display = "I2C Error")]
    I2CError(std::io::ErrorKind),
    #[fail(display = "I2C Error")]
    I2CError2(u8),
    /// I2C Set Error
    #[fail(display = "I2C Set Error")]
    I2CSet,
    #[fail(display = "Transfer error")]
    TransferError,
    #[fail(display = "InvalidInput error")]
    InvalidInput,
    // // Errors from deserialization
    // #[fail(display = "bincode Error")]
    // Bincode(u8),
    // Response Status Information (STAT) Errors
    #[fail(display = "Rejected")]
    Rejected,
    #[fail(display = "Rejected: Invalid command code error")]
    InvalidCommandCode,
    #[fail(display = "Rejected: Parameter missing error")]
    ParameterMissing,
    #[fail(display = "Rejected: Parameter invalid error")]
    Parameterinvalid,
    #[fail(display = "Rejected: Unavailable in current mode/configuration error")]
    UnavailableMode,
    #[fail(display = "Rejected: Invalid system type, interface version, or BID error")]
    InvalidSystemType,
    #[fail(display = "Internal error occurred during processing")]
    InternalProcessing,
    #[fail(display = "Invalid Reset Cause")]
    InvalidResetCause,
    #[fail(display = "Invalid Eps Mode")]
    InvalidEpsMode,
    #[fail(display = "Invalid Bus Channel State")]
    InvalidBusChannelState,
}

/// All Errors in EpsError are converted to Error::ServiceError(u8)
impl From<EpsError> for Error {
    fn from(e: EpsError) -> Error {
        match e {
            EpsError::Err => Error::ServiceError(0),
            EpsError::I2CError(io) => Error::from(io),
            EpsError::I2CError2(io) => Error::Io(io),
            EpsError::I2CSet => Error::ServiceError(1),
            EpsError::TransferError => Error::ServiceError(2),
            EpsError::InvalidInput => Error::ServiceError(3),
            // EpsError::Bincode(io) => Error::Bincode(io),
            EpsError::Rejected => Error::ServiceError(4),
            EpsError::InvalidCommandCode => Error::ServiceError(5),
            EpsError::ParameterMissing => Error::ServiceError(6),
            EpsError::Parameterinvalid => Error::ServiceError(7),
            EpsError::UnavailableMode => Error::ServiceError(8),
            EpsError::InvalidSystemType => Error::ServiceError(9),
            EpsError::InternalProcessing => Error::ServiceError(10),
            EpsError::InvalidResetCause => Error::ServiceError(11),
            EpsError::InvalidEpsMode => Error::ServiceError(12),
            EpsError::InvalidBusChannelState => Error::ServiceError(13),
            // _ => Error::ServiceError(0),
        }
    }
}

impl From<Error> for EpsError {
    fn from(e: Error) -> EpsError {
        match e {
            Error::ServiceError(0) => EpsError::Err,
            Error::Io(io) => EpsError::I2CError2(io),
            Error::ServiceError(1) => EpsError::I2CSet,
            Error::ServiceError(2) => EpsError::TransferError,
            Error::ServiceError(3) => EpsError::InvalidInput,
            // Error::Bincode(io) => EpsError::Bincode(io),
            Error::ServiceError(4) => EpsError::Rejected,
            Error::ServiceError(5) => EpsError::InvalidCommandCode,
            Error::ServiceError(6) => EpsError::ParameterMissing,
            Error::ServiceError(7) => EpsError::Parameterinvalid,
            Error::ServiceError(8) => EpsError::UnavailableMode,
            Error::ServiceError(9) => EpsError::InvalidSystemType,
            Error::ServiceError(10) => EpsError::InternalProcessing,
            Error::ServiceError(11) => EpsError::InvalidResetCause,
            Error::ServiceError(12) => EpsError::InvalidEpsMode,
            Error::ServiceError(13) => EpsError::InvalidBusChannelState,
            _ => EpsError::Err,
        }
    }
}

// impl From<bincode::Error> for EpsError {
//     fn from(b: bincode::Error) -> EpsError {
//         match *b {
//             bincode::ErrorKind::Io(_) => EpsError::Bincode(0),
//             bincode::ErrorKind::InvalidUtf8Encoding(_) => EpsError::Bincode(1),
//             bincode::ErrorKind::InvalidBoolEncoding(_) => EpsError::Bincode(2),
//             bincode::ErrorKind::InvalidCharEncoding => EpsError::Bincode(3),
//             bincode::ErrorKind::InvalidTagEncoding(_) => EpsError::Bincode(4),
//             bincode::ErrorKind::DeserializeAnyNotSupported => EpsError::Bincode(5),
//             bincode::ErrorKind::SizeLimit => EpsError::Bincode(6),
//             bincode::ErrorKind::SequenceMustHaveLength => EpsError::Bincode(7),
//             bincode::ErrorKind::Custom(_) => EpsError::Bincode(8),
//         }
//     }
// }

// Result type to be implemented
pub type EpsResult<T> = Result<T, EpsError>;
