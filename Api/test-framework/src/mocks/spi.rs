/*
 * Mock SPI Interface Implementation
 * Copyright (C) 2024
 */

use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Bidirectional};
use crate::interfaces::spi::SPIConfig;
use async_trait::async_trait;
use mockall::mock;
use std::time::Duration;

mock! {
    pub SPIInterface {
        pub fn new(config: SPIConfig) -> Self;
        pub fn with_default_config() -> Self;
        pub fn get_speed(&self) -> u32;
        pub fn set_speed(&mut self, speed: u32);
        pub fn get_mode(&self) -> u8;
        pub fn set_mode(&mut self, mode: u8);
        pub fn get_bits_per_word(&self) -> u8;
        pub fn set_bits_per_word(&mut self, bits: u8);
    }
    
    #[async_trait]
    impl HardwareInterface for SPIInterface {
        async fn initialize(&mut self) -> HardwareResult<()>;
        async fn deinitialize(&mut self) -> HardwareResult<()>;
        fn is_initialized(&self) -> bool;
        async fn get_status(&self) -> HardwareResult<InterfaceStatus>;
    }
    
    #[async_trait]
    impl Bidirectional for SPIInterface {
        async fn transfer(&mut self, tx_data: &[u8], rx_data: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
    }
}

impl MockSPIInterface {
    pub fn new_with_defaults() -> Self {
        let mut mock = Self::new(SPIConfig::default());
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
    async fn test_mock_spi_interface() {
        let mut mock = MockSPIInterface::new(SPIConfig::default());
        
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
    async fn test_mock_spi_transfer() {
        let mut mock = MockSPIInterface::new(SPIConfig::default());
        
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
    async fn test_mock_spi_config() {
        let mut mock = MockSPIInterface::new(SPIConfig::default());
        
        mock.expect_get_speed()
            .returning(|| 1_000_000);
        mock.expect_set_speed()
            .with(eq(2_000_000))
            .times(1);
        mock.expect_get_mode()
            .returning(|| 0);
        mock.expect_set_mode()
            .with(eq(3))
            .times(1);
        mock.expect_get_bits_per_word()
            .returning(|| 8);
        mock.expect_set_bits_per_word()
            .with(eq(16))
            .times(1);
            
        assert_eq!(mock.get_speed(), 1_000_000);
        mock.set_speed(2_000_000);
        
        assert_eq!(mock.get_mode(), 0);
        mock.set_mode(3);
        
        assert_eq!(mock.get_bits_per_word(), 8);
        mock.set_bits_per_word(16);
    }
    
    #[tokio::test]
    async fn test_mock_spi_error_handling() {
        let mut mock = MockSPIInterface::new(SPIConfig::default());
        
        mock.expect_initialize()
            .times(1)
            .returning(|| Err(crate::HardwareError::DeviceNotFound));
            
        assert!(matches!(
            mock.initialize().await,
            Err(crate::HardwareError::DeviceNotFound)
        ));
        
        // Test mismatched buffer sizes
        let mut mock = MockSPIInterface::new(SPIConfig::default());
        mock.expect_initialize()
            .returning(|| Ok(()));
        mock.expect_transfer()
            .with(eq(vec![1, 2, 3]), eq(4), eq(Duration::from_millis(100)))
            .times(1)
            .returning(|_, _, _| Err(crate::HardwareError::InvalidParameter(
                "TX and RX buffers must be the same size".to_string()
            )));
            
        assert!(mock.initialize().await.is_ok());
        
        let tx_data = vec![1, 2, 3];
        let mut rx_data = vec![0u8; 4];
        assert!(matches!(
            mock.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await,
            Err(crate::HardwareError::InvalidParameter(_))
        ));
    }
} 