/*
 * SPI Interface Implementation
 * Copyright (C) 2024
 */

use super::{InterfaceParams, InterfaceState};
use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Bidirectional};
use async_trait::async_trait;
use std::time::Duration;

/// SPI interface configuration
#[derive(Debug, Clone)]
pub struct SPIConfig {
    pub device_path: String,
    pub mode: u8,
    pub speed: u32,
    pub bits_per_word: u8,
    pub params: InterfaceParams,
}

impl Default for SPIConfig {
    fn default() -> Self {
        Self {
            device_path: "/dev/spidev0.0".to_string(),
            mode: 0,
            speed: 1_000_000,
            bits_per_word: 8,
            params: InterfaceParams::default(),
        }
    }
}

/// SPI interface implementation
pub struct SPIInterface {
    config: SPIConfig,
    state: InterfaceState,
    handle: Option<i32>,
}

impl SPIInterface {
    pub fn new(config: SPIConfig) -> Self {
        Self {
            config,
            state: InterfaceState::new(),
            handle: None,
        }
    }
    
    pub fn with_default_config() -> Self {
        Self::new(SPIConfig::default())
    }
    
    async fn open_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would open the SPI device
        // For testing, we'll just simulate success
        self.handle = Some(1);
        Ok(())
    }
    
    async fn close_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would close the SPI device
        // For testing, we'll just simulate success
        self.handle = None;
        Ok(())
    }
    
    pub fn get_speed(&self) -> u32 {
        self.config.speed
    }
    
    pub fn set_speed(&mut self, speed: u32) {
        self.config.speed = speed;
    }
    
    pub fn get_mode(&self) -> u8 {
        self.config.mode
    }
    
    pub fn set_mode(&mut self, mode: u8) {
        self.config.mode = mode;
    }
}

#[async_trait]
impl HardwareInterface for SPIInterface {
    async fn initialize(&mut self) -> HardwareResult<()> {
        if self.state.initialized {
            return Ok(());
        }
        
        match self.open_device().await {
            Ok(_) => {
                self.state.initialized = true;
                Ok(())
            }
            Err(e) => {
                self.state.record_error(e.to_string());
                Err(e)
            }
        }
    }
    
    async fn deinitialize(&mut self) -> HardwareResult<()> {
        if !self.state.initialized {
            return Ok(());
        }
        
        match self.close_device().await {
            Ok(_) => {
                self.state.initialized = false;
                Ok(())
            }
            Err(e) => {
                self.state.record_error(e.to_string());
                Err(e)
            }
        }
    }
    
    fn is_initialized(&self) -> bool {
        self.state.initialized
    }
    
    async fn get_status(&self) -> HardwareResult<InterfaceStatus> {
        Ok(self.state.to_status())
    }
}

#[async_trait]
impl Bidirectional for SPIInterface {
    async fn transfer(&mut self, tx_data: &[u8], rx_data: &mut [u8], timeout: Duration) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        if tx_data.len() != rx_data.len() {
            return Err(crate::HardwareError::InvalidParameter(
                "TX and RX buffers must be the same size".to_string()
            ));
        }
        
        // In a real implementation, this would perform an SPI transfer
        // For testing, we'll just simulate success
        Ok(rx_data.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    
    #[tokio::test]
    async fn test_spi_initialization() {
        let mut interface = SPIInterface::with_default_config();
        
        assert!(!interface.is_initialized());
        assert!(interface.initialize().await.is_ok());
        assert!(interface.is_initialized());
        assert!(interface.deinitialize().await.is_ok());
        assert!(!interface.is_initialized());
    }
    
    #[tokio::test]
    async fn test_spi_transfer() {
        let mut interface = SPIInterface::with_default_config();
        assert!(interface.initialize().await.is_ok());
        
        let tx_data = vec![1, 2, 3, 4, 5];
        let mut rx_data = vec![0u8; 5];
        
        assert_eq!(interface.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await.unwrap(), 5);
    }
    
    #[tokio::test]
    async fn test_spi_config() {
        let mut interface = SPIInterface::with_default_config();
        
        assert_eq!(interface.get_speed(), 1_000_000);
        interface.set_speed(2_000_000);
        assert_eq!(interface.get_speed(), 2_000_000);
        
        assert_eq!(interface.get_mode(), 0);
        interface.set_mode(3);
        assert_eq!(interface.get_mode(), 3);
    }
    
    #[tokio::test]
    async fn test_spi_error_handling() {
        let mut interface = SPIInterface::with_default_config();
        
        // Test uninitialized operations
        let tx_data = vec![1, 2, 3];
        let mut rx_data = vec![0u8; 3];
        
        assert!(matches!(
            interface.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await,
            Err(crate::HardwareError::NotInitialized)
        ));
        
        // Test mismatched buffer sizes
        assert!(interface.initialize().await.is_ok());
        let mut rx_data = vec![0u8; 4];
        assert!(matches!(
            interface.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await,
            Err(crate::HardwareError::InvalidParameter(_))
        ));
    }
} 