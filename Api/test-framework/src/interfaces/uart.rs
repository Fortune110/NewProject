/*
 * UART Interface Implementation
 * Copyright (C) 2024
 */

use super::{InterfaceParams, InterfaceState};
use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Readable, Writable};
use async_trait::async_trait;
use std::time::Duration;

/// UART interface configuration
#[derive(Debug, Clone)]
pub struct UARTConfig {
    pub device_path: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: Parity,
    pub flow_control: FlowControl,
    pub params: InterfaceParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowControl {
    None,
    Hardware,
    Software,
}

impl Default for UARTConfig {
    fn default() -> Self {
        Self {
            device_path: "/dev/ttyUSB0".to_string(),
            baud_rate: 9600,
            data_bits: 8,
            stop_bits: 1,
            parity: Parity::None,
            flow_control: FlowControl::None,
            params: InterfaceParams::default(),
        }
    }
}

/// UART interface implementation
pub struct UARTInterface {
    config: UARTConfig,
    state: InterfaceState,
    handle: Option<i32>,
}

impl UARTInterface {
    pub fn new(config: UARTConfig) -> Self {
        Self {
            config,
            state: InterfaceState::new(),
            handle: None,
        }
    }
    
    pub fn with_default_config() -> Self {
        Self::new(UARTConfig::default())
    }
    
    async fn open_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would open the UART device
        // For testing, we'll just simulate success
        self.handle = Some(1);
        Ok(())
    }
    
    async fn close_device(&mut self) -> HardwareResult<()> {
        // In a real implementation, this would close the UART device
        // For testing, we'll just simulate success
        self.handle = None;
        Ok(())
    }
    
    pub fn get_baud_rate(&self) -> u32 {
        self.config.baud_rate
    }
    
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.config.baud_rate = baud_rate;
    }
}

#[async_trait]
impl HardwareInterface for UARTInterface {
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
impl Readable for UARTInterface {
    async fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        // In a real implementation, this would read from the UART device
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
impl Writable for UARTInterface {
    async fn write(&mut self, data: &[u8]) -> HardwareResult<usize> {
        if !self.state.initialized {
            return Err(crate::HardwareError::NotInitialized);
        }
        
        // In a real implementation, this would write to the UART device
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    
    #[tokio::test]
    async fn test_uart_initialization() {
        let mut interface = UARTInterface::with_default_config();
        
        assert!(!interface.is_initialized());
        assert!(interface.initialize().await.is_ok());
        assert!(interface.is_initialized());
        assert!(interface.deinitialize().await.is_ok());
        assert!(!interface.is_initialized());
    }
    
    #[tokio::test]
    async fn test_uart_read_write() {
        let mut interface = UARTInterface::with_default_config();
        assert!(interface.initialize().await.is_ok());
        
        let test_data = vec![1, 2, 3, 4, 5];
        let mut read_buffer = vec![0u8; 5];
        
        assert_eq!(interface.write(&test_data).await.unwrap(), 5);
        assert_eq!(interface.read(&mut read_buffer, Duration::from_millis(100)).await.unwrap(), 5);
    }
    
    #[tokio::test]
    async fn test_uart_config() {
        let mut interface = UARTInterface::with_default_config();
        
        assert_eq!(interface.get_baud_rate(), 9600);
        interface.set_baud_rate(115200);
        assert_eq!(interface.get_baud_rate(), 115200);
    }
    
    #[tokio::test]
    async fn test_uart_error_handling() {
        let mut interface = UARTInterface::with_default_config();
        
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
    }
} 