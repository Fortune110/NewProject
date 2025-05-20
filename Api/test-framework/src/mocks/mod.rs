/*
 * Mock Implementations for Hardware Interface Testing
 * Copyright (C) 2024
 */

mod i2c;
mod uart;
mod spi;

pub use i2c::MockI2CInterface;
pub use uart::MockUARTInterface;
pub use spi::MockSPIInterface;

use crate::{HardwareInterface, HardwareResult, InterfaceStatus};
use async_trait::async_trait;
use mockall::mock;

/// Mock hardware interface for testing
mock! {
    pub HardwareInterface {}
    
    #[async_trait]
    impl HardwareInterface for HardwareInterface {
        async fn initialize(&mut self) -> HardwareResult<()>;
        async fn deinitialize(&mut self) -> HardwareResult<()>;
        fn is_initialized(&self) -> bool;
        async fn get_status(&self) -> HardwareResult<InterfaceStatus>;
    }
}

/// Helper function to create a mock hardware interface
pub fn create_mock_interface() -> MockHardwareInterface {
    MockHardwareInterface::new()
}

/// Helper function to create a mock hardware interface with default expectations
pub fn create_mock_interface_with_defaults() -> MockHardwareInterface {
    let mut mock = MockHardwareInterface::new();
    mock.expect_initialize()
        .returning(|| Ok(()));
    mock.expect_is_initialized()
        .returning(|| true);
    mock.expect_deinitialize()
        .returning(|| Ok(()));
    mock.expect_get_status()
        .returning(|| Ok(InterfaceStatus {
            initialized: true,
            error_count: 0,
            last_error: None,
            uptime: std::time::Duration::from_secs(0),
        }));
    mock
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_mock_interface() {
        let mut mock = create_mock_interface();
        
        mock.expect_initialize()
            .times(1)
            .returning(|| Ok(()));
        mock.expect_is_initialized()
            .times(1)
            .returning(|| true);
        mock.expect_deinitialize()
            .times(1)
            .returning(|| Ok(()));
            
        assert!(mock.initialize().await.is_ok());
        assert!(mock.is_initialized());
        assert!(mock.deinitialize().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_mock_interface_with_defaults() {
        let mock = create_mock_interface_with_defaults();
        
        assert!(mock.initialize().await.is_ok());
        assert!(mock.is_initialized());
        assert!(mock.deinitialize().await.is_ok());
        
        let status = mock.get_status().await.unwrap();
        assert!(status.initialized);
        assert_eq!(status.error_count, 0);
        assert!(status.last_error.is_none());
    }
} 