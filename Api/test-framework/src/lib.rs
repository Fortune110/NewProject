/*
 * Hardware Interface Test Framework
 * Copyright (C) 2024
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod interfaces;
mod mocks;
mod runner;
mod utils;

pub use interfaces::*;
pub use mocks::*;
pub use runner::*;
pub use utils::*;

use std::fmt;
use std::time::Duration;
use async_trait::async_trait;
use thiserror::Error;
use std::error::Error;

/// Hardware interface error types
#[derive(Debug, PartialEq)]
pub enum HardwareError {
    CommunicationError(String),
    TimeoutError,
    InvalidParameter(String),
    DeviceNotFound,
    PermissionDenied,
    NotInitialized,
    AlreadyInitialized,
    OperationFailed(String),
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HardwareError::CommunicationError(msg) => write!(f, "Communication error: {}", msg),
            HardwareError::TimeoutError => write!(f, "Operation timed out"),
            HardwareError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            HardwareError::DeviceNotFound => write!(f, "Device not found"),
            HardwareError::PermissionDenied => write!(f, "Permission denied"),
            HardwareError::NotInitialized => write!(f, "Device not initialized"),
            HardwareError::AlreadyInitialized => write!(f, "Device already initialized"),
            HardwareError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl Error for HardwareError {}

/// Result type for hardware operations
pub type HardwareResult<T> = Result<T, HardwareError>;

/// Interface status information
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceStatus {
    pub is_initialized: bool,
    pub error_count: u32,
    pub warning_count: u32,
    pub last_error: Option<String>,
}

/// Common interface parameters
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceParams {
    pub timeout: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
}

impl Default for InterfaceParams {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(1),
            retry_count: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

/// Hardware interface trait
pub trait HardwareInterface {
    /// Initialize the interface
    fn initialize(&mut self) -> HardwareResult<()>;
    
    /// Deinitialize the interface
    fn deinitialize(&mut self) -> HardwareResult<()>;
    
    /// Check if the interface is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get the interface status
    fn get_status(&self) -> InterfaceStatus;
}

/// Readable interface trait
pub trait Readable {
    /// Read data from the interface
    fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
}

/// Writable interface trait
pub trait Writable {
    /// Write data to the interface
    fn write(&mut self, data: &[u8]) -> HardwareResult<usize>;
}

/// Bidirectional interface trait
pub trait Bidirectional: Readable + Writable {
    /// Transfer data in both directions
    fn transfer(&mut self, tx_data: &[u8], rx_buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
}

/// Configuration for hardware interfaces
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub timeout: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
    pub debug_mode: bool,
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(1000),
            retry_count: 3,
            retry_delay: Duration::from_millis(100),
            debug_mode: false,
        }
    }
}

/// Test context for hardware interface tests
pub struct TestContext<T: HardwareInterface> {
    pub interface: T,
    pub config: InterfaceConfig,
}

impl<T: HardwareInterface> TestContext<T> {
    pub fn new(interface: T, config: InterfaceConfig) -> Self {
        Self { interface, config }
    }
    
    pub async fn setup(&mut self) -> HardwareResult<()> {
        self.interface.initialize().await
    }
    
    pub async fn teardown(&mut self) -> HardwareResult<()> {
        self.interface.deinitialize().await
    }
}

/// Test utilities for black-box testing
pub mod test_utils {
    use super::*;
    use rstest::rstest;

    // Helper function to create test data
    pub fn create_test_data(size: usize) -> Vec<u8> {
        (0..size).map(|i| i as u8).collect()
    }

    // Helper function to verify test data
    pub fn verify_test_data(data: &[u8], expected_size: usize) -> bool {
        data.len() == expected_size && data.iter().enumerate().all(|(i, &v)| v == i as u8)
    }

    // Helper function to run test with retries
    pub async fn run_with_retries<F, Fut>(f: F, retry_count: u32, retry_delay: Duration) -> HardwareResult<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = HardwareResult<()>>,
    {
        let mut attempts = 0;
        loop {
            match f().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    attempts += 1;
                    if attempts >= retry_count {
                        return Err(e);
                    }
                    tokio::time::sleep(retry_delay).await;
                }
            }
        }
    }

    // Helper function to run test with timeout
    pub async fn run_with_timeout<F, Fut>(f: F, timeout: Duration) -> HardwareResult<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = HardwareResult<()>>,
    {
        tokio::time::timeout(timeout, f())
            .await
            .map_err(|_| HardwareError::TimeoutError)?
    }

    // Helper function to run test with retries and timeout
    pub async fn run_with_retries_and_timeout<F, Fut>(
        f: F,
        retry_count: u32,
        retry_delay: Duration,
        timeout: Duration,
    ) -> HardwareResult<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = HardwareResult<()>>,
    {
        run_with_timeout(|| run_with_retries(f, retry_count, retry_delay), timeout).await
    }
}

// Re-export commonly used items
pub use test_utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Test data generation
    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(10)]
    #[case(100)]
    fn test_create_test_data(#[case] size: usize) {
        let data = create_test_data(size);
        assert_eq!(data.len(), size);
        assert!(verify_test_data(&data, size));
    }

    // Test data verification
    #[rstest]
    #[case(vec![0, 1, 2, 3], 4, true)]
    #[case(vec![0, 1, 2], 4, false)]
    #[case(vec![0, 1, 2, 4], 4, false)]
    fn test_verify_test_data(#[case] data: Vec<u8>, #[case] expected_size: usize, #[case] expected: bool) {
        assert_eq!(verify_test_data(&data, expected_size), expected);
    }
} 