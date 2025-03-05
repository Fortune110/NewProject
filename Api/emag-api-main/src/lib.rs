mod commands;
mod emag;
pub mod objects;

use std::io;

pub use crate::commands::*;
pub use crate::emag::*;
pub use crate::objects::*;

/// Universal return type for Radiation Counter API functions
pub type EmagResult<T> = Result<T, io::Error>;
