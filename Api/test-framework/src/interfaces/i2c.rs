/*
 * I2C Interface Implementation
 * Copyright (C) 2024
 */

use super::{InterfaceParams, InterfaceState};
use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Readable, Writable, Bidirectional};
use async_trait::async_trait;
use std::time::Duration;

/// I2C interface configuration
#[derive(Debug, Clone)]
pub struct I2CConfig {
    pub bus_number: u8,
    pub device_address: u16,
    pub clock_speed: u32,
    pub params: InterfaceParams,
}

impl Default for I2CConfig {
    fn default() -> Self {
        Self {
            bus_number: 1,
            device_address: 0x50,
            clock_speed: 100_000,
            params: InterfaceParams::default(),
        }
    }
}

/// I2C interface implementation
pub struct I2CInterface {
    config: I2CConfig,
    state: InterfaceState,
    handle: Option<i32>,
}

impl I2CInterface {
    pub fn new(config: I2CConfig) -> Self {
        Self {
            config,
            state: InterfaceState::new(),
            handle: None,
        }
    }
    
    pub fn with_default_config() -> Self {
        Self::new(I2CConfig::default())
    }
    
    fn get_device_path(&self) -> String {
        format!("/dev/i2c-{}", self.config.bus_number)
    }
    
    async fn open_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would open the I2C device
        // For testing, we'll just simulate success
        self.handle = Some(1);
        Ok(())
    }
    
    async fn close_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would close the I2C device
        // For testing, we'll just simulate success
        self.handle = None;
        Ok(())
    }
}

#[async_trait]
impl HardwareInterface for I2CInterface {
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
impl Readable for I2CInterface {
    async fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        // In a real implementation, this would read from the I2C device
        // For testing, we'll just simulate success
        Ok(buffer.len())
    }
    
    async fn read_exact(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<()> {
        let bytes_read = self.read(buffer, timeout).await?;
        if bytes_read != buffer.len() {
            return Err(crate::HardwareError::CommunicationError(
                "Failed to read exact number of bytes".to_string()
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl Writable for I2CInterface {
    async fn write(&mut self, data: &[u8]) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        // In a real implementation, this would write to the I2C device
        // For testing, we'll just simulate success
        Ok(data.len())
    }
    
    async fn write_all(&mut self, data: &[u8]) -> HardwareResult<()> {
        let bytes_written = self.write(data).await?;
        if bytes_written != data.len() {
            return Err(crate::HardwareError::CommunicationError(
                "Failed to write all bytes".to_string()
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl Bidirectional for I2CInterface {
    async fn transfer(&mut self, tx_data: &[u8], rx_data: &mut [u8], timeout: Duration) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        // In a real implementation, this would perform an I2C transfer
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
    async fn test_i2c_initialization() {
        let mut interface = I2CInterface::with_default_config();
        
        assert!(!interface.is_initialized());
        assert!(interface.initialize().await.is_ok());
        assert!(interface.is_initialized());
        assert!(interface.deinitialize().await.is_ok());
        assert!(!interface.is_initialized());
    }
    
    #[tokio::test]
    async fn test_i2c_read_write() {
        let mut interface = I2CInterface::with_default_config();
        assert!(interface.initialize().await.is_ok());
        
        let test_data = vec![1, 2, 3, 4, 5];
        let mut read_buffer = vec![0u8; 5];
        
        assert_eq!(interface.write(&test_data).await.unwrap(), 5);
        assert_eq!(interface.read(&mut read_buffer, Duration::from_millis(100)).await.unwrap(), 5);
    }
    
    #[tokio::test]
    async fn test_i2c_transfer() {
        let mut interface = I2CInterface::with_default_config();
        assert!(interface.initialize().await.is_ok());
        
        let tx_data = vec![1, 2, 3, 4, 5];
        let mut rx_data = vec![0u8; 5];
        
        assert_eq!(interface.transfer(&tx_data, &mut rx_data, Duration::from_millis(100)).await.unwrap(), 5);
    }
    
    #[tokio::test]
    async fn test_i2c_error_handling() {
        let mut interface = I2CInterface::with_default_config();
        
        // Test uninitialized operations
        let mut buffer = vec![0u8; 5];
        assert!(matches!(
            interface.read(&mut buffer, Duration::from_millis(100)).await,
            Err(crate::HardwareError::NotInitialized)
        ));
        
        assert!(matches!(
            interface.write(&[1, 2, 3]).await,
            Err(crate::HardwareError::NotInitialized)
        ));
        
        assert!(matches!(
            interface.transfer(&[1, 2, 3], &mut buffer, Duration::from_millis(100)).await,
            Err(crate::HardwareError::NotInitialized)
        ));
    }
} 