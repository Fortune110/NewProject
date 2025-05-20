/*
 * Mock UART Interface Implementation
 * Copyright (C) 2024
 */

use crate::{HardwareInterface, HardwareResult, InterfaceStatus, Readable, Writable};
use crate::interfaces::uart::{UARTConfig, Parity, FlowControl};
use async_trait::async_trait;
use mockall::mock;
use std::time::Duration;

mock! {
    pub UARTInterface {
        pub fn new(config: UARTConfig) -> Self;
        pub fn with_default_config() -> Self;
        pub fn get_baud_rate(&self) -> u32;
        pub fn set_baud_rate(&mut self, baud_rate: u32);
        pub fn get_parity(&self) -> Parity;
        pub fn set_parity(&mut self, parity: Parity);
        pub fn get_flow_control(&self) -> FlowControl;
        pub fn set_flow_control(&mut self, flow_control: FlowControl);
    }
    
    #[async_trait]
    impl HardwareInterface for UARTInterface {
        async fn initialize(&mut self) -> HardwareResult<()>;
        async fn deinitialize(&mut self) -> HardwareResult<()>;
        fn is_initialized(&self) -> bool;
        async fn get_status(&self) -> HardwareResult<InterfaceStatus>;
    }
    
    #[async_trait]
    impl Readable for UARTInterface {
        async fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<usize>;
        async fn read_exact(&mut self, buffer: &mut [u8], timeout: Duration) -> HardwareResult<()>;
    }
    
    #[async_trait]
    impl Writable for UARTInterface {
        async fn write(&mut self, data: &[u8]) -> HardwareResult<usize>;
        async fn write_all(&mut self, data: &[u8]) -> HardwareResult<()>;
    }
}

impl MockUARTInterface {
    pub fn new_with_defaults() -> Self {
        let mut mock = Self::new(UARTConfig::default());
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
    async fn test_mock_uart_interface() {
        let mut mock = MockUARTInterface::new(UARTConfig::default());
        
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
    async fn test_mock_uart_read_write() {
        let mut mock = MockUARTInterface::new(UARTConfig::default());
        
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
    async fn test_mock_uart_config() {
        let mut mock = MockUARTInterface::new(UARTConfig::default());
        
        mock.expect_get_baud_rate()
            .returning(|| 9600);
        mock.expect_set_baud_rate()
            .with(eq(115200))
            .times(1);
        mock.expect_get_parity()
            .returning(|| Parity::None);
        mock.expect_set_parity()
            .with(eq(Parity::Even))
            .times(1);
        mock.expect_get_flow_control()
            .returning(|| FlowControl::None);
        mock.expect_set_flow_control()
            .with(eq(FlowControl::Hardware))
            .times(1);
            
        assert_eq!(mock.get_baud_rate(), 9600);
        mock.set_baud_rate(115200);
        
        assert_eq!(mock.get_parity(), Parity::None);
        mock.set_parity(Parity::Even);
        
        assert_eq!(mock.get_flow_control(), FlowControl::None);
        mock.set_flow_control(FlowControl::Hardware);
    }
    
    #[tokio::test]
    async fn test_mock_uart_error_handling() {
        let mut mock = MockUARTInterface::new(UARTConfig::default());
        
        mock.expect_initialize()
            .times(1)
            .returning(|| Err(crate::HardwareError::DeviceNotFound));
            
        assert!(matches!(
            mock.initialize().await,
            Err(crate::HardwareError::DeviceNotFound)
        ));
    }
} 