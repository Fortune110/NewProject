/*
 * Hardware Interface Implementations
 * Copyright (C) 2024
 */

mod i2c;
mod uart;
mod spi;

pub use i2c::I2CInterface;
pub use uart::UARTInterface;
pub use spi::SPIInterface;

use crate::{HardwareInterface, HardwareResult, InterfaceStatus};
use std::time::Duration;
use async_trait::async_trait;

/// Common interface parameters
#[derive(Debug, Clone)]
pub struct InterfaceParams {
    pub device_path: String,
    pub timeout: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
}

impl Default for InterfaceParams {
    fn default() -> Self {
        Self {
            device_path: String::new(),
            timeout: Duration::from_millis(1000),
            retry_count: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

/// Common interface state
#[derive(Debug, Clone)]
pub struct InterfaceState {
    pub initialized: bool,
    pub error_count: u32,
    pub last_error: Option<String>,
    pub start_time: std::time::Instant,
}

impl Default for InterfaceState {
    fn default() -> Self {
        Self {
            initialized: false,
            error_count: 0,
            last_error: None,
            start_time: std::time::Instant::now(),
        }
    }
}

impl InterfaceState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_error(&mut self, error: String) {
        self.error_count += 1;
        self.last_error = Some(error);
    }
    
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn to_status(&self) -> InterfaceStatus {
        InterfaceStatus {
            initialized: self.initialized,
            error_count: self.error_count,
            last_error: self.last_error.as_ref().map(|e| crate::HardwareError::CommunicationError(e.clone())),
            uptime: self.get_uptime(),
        }
    }
} 