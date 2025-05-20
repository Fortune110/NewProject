/*
 * Mock I2C Interface Implementation
 * Copyright (C) 2024
 */

use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Readable, Writable, Bidirectional};
use crate::interfaces::i2c::I2CConfig;
use async_trait::async_trait;
use mockall::mock;
use std::time::Duration;

mock! {
    pub I2CInterface {
        pub fn new(config: I2CConfig) -> Self;
        pub fn with_default_config() -> Self;
        pub fn get_device_address(&self) -> u16;
        pub fn set_device_address(&mut self, address: u16);
        pub fn get_clock_speed(&self) -> u32;
        pub fn set_clock_speed(&mut self, speed: u32);
    }
    
    #[async_trait]
    impl HardwareInterface for I2CInterface {
        async fn initialize(&mut self) -> HardwareResult<()>;
        async fn deinitialize(&mut self) -> HardwareResult<()>;
        fn is_initialized(&self) -> bool;
        async fn get_status(&self) -> HardwareResult<InterfaceStatus>;
    }
    
    #[async_trait]
    impl Readable for I2CInterface {
        async fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
        async fn read_exact(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<()>;
    }
    
    #[async_trait]
    impl Writable for I2CInterface {
        async fn write(&mut self, data: &[u8]) -> HardwareResult<usize>;
        async fn write_all(&mut self, data: &[u8]) -> HardwareResult<()>;
    }
    
    #[async_trait]
    impl Bidirectional for I2CInterface {
        async fn transfer(&mut self, tx_data: &[u8], rx_data: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
    }
}

impl MockI2CInterface {
    pub fn new_with_defaults() -> Self {
        let mut mock = Self::new(I2CConfig::default());
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
                uptime: Duration::from_secs(0),
            }));
        mock
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_mock_i2c_interface() {
        let mut mock = MockI2CInterface::new(I2CConfig::default());
        
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
    async fn test_mock_i2c_read_write() {
        let mut mock = MockI2CInterface::new(I2CConfig::default());
        
        mock.expect_initialize()
            .returning(|| Ok(()));
        mock.expect_write()
            .with(eq(vec![1, 2, 3]))
            .times(1)
            .returning(|_| Ok(3));
        mock.expect_read()
            .with(eq(3), eq(Duration::from_millis(100)))
            .times(1)
            .returning(|_, _| Ok(3));
            
        assert!(mock.initialize().await.is_ok());
        assert_eq!(mock.write(&[1, 2, 3]).await.unwrap(), 3);
        
        let mut buffer = vec![0u8; 3];
        assert_eq!(mock.read(&mut buffer, Duration::from_millis(100)).await.unwrap(), 3);
    }
    
    #[tokio::test]
    async fn test_mock_i2c_transfer() {
        let mut mock = MockI2CInterface::new(I2CConfig::default());
        
        mock.expect_initialize()
            .returning(|| Ok(()));
        mock.expect_transfer()
            .with(eq(vec![1, 2, 3]), eq(3), eq(Duration::from_millis(100)))
            .times(1)
            .returning(|_, _, _| Ok(3));
            
        assert!(mock.initialize().await.is_ok());
        
        let tx_data = vec![1, 2, 3];
        let mut rx_data = vec![0u8; 3];
        assert_eq!(mock.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await.unwrap(), 3);
    }
    
    #[tokio::test]
    async fn test_mock_i2c_error_handling() {
        let mut mock = MockI2CInterface::new(I2CConfig::default());
        
        mock.expect_initialize()
            .times(1)
            .returning(|| Err(crate::HardwareError::DeviceNotFound));
            
        assert!(matches!(
            mock.initialize().await,
            Err(crate::HardwareError::DeviceNotFound)
        ));
    }
} 