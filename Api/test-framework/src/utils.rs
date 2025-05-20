/*
 * Test Utilities for Hardware Interface Testing
 * Copyright (C) 2024
 */

use crate::{HardwareInterface, HardwareResult, InterfaceStatus};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;

/// Test context for hardware interface tests
pub struct TestContext<T: HardwareInterface> {
    pub interface: Arc<Mutex<T>>,
    pub timeout: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
}

impl<T: HardwareInterface> TestContext<T> {
    pub fn new(interface: T, timeout: Duration, retry_count: u32, retry_delay: Duration) -> Self {
        Self {
            interface: Arc::new(Mutex::new(interface)),
            timeout,
            retry_count,
            retry_delay,
        }
    }
    
    pub async fn setup(&self) -> HardwareResult<()> {
        let mut interface = self.interface.lock().await;
        interface.initialize().await
    }
    
    pub async fn teardown(&self) -> HardwareResult<()> {
        let mut interface = self.interface.lock().await;
        interface.deinitialize().await
    }
    
    pub async fn get_status(&self) -> HardwareResult<InterfaceStatus> {
        let interface = self.interface.lock().await;
        interface.get_status().await
    }
}

/// Helper function to create test data
pub fn create_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| i as u8).collect()
}

/// Helper function to verify test data
pub fn verify_test_data(data: &[u8]) -> bool {
    data.iter().enumerate().all(|(i, &byte)| byte == i as u8)
}

/// Helper function to run a test with retries
pub async fn run_with_retries<F, T, E>(f: F, retry_count: u32, retry_delay: Duration) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Debug,
{
    let mut last_error = None;
    
    for _ in 0..retry_count {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                time::sleep(retry_delay).await;
            }
        }
    }
    
    Err(last_error.unwrap())
}

/// Helper function to run a test with timeout
pub async fn run_with_timeout<F, T>(f: F, timeout: Duration) -> Result<T, HardwareResult<()>>
where
    F: std::future::Future<Output = T>,
{
    match time::timeout(timeout, f).await {
        Ok(result) => Ok(result),
        Err(_) => Err(crate::HardwareError::TimeoutError.into()),
    }
}

/// Helper function to run a test with both retries and timeout
pub async fn run_with_retries_and_timeout<F, T, E>(
    f: F,
    retry_count: u32,
    retry_delay: Duration,
    timeout: Duration,
) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Debug,
{
    run_with_timeout(
        async move { run_with_retries(f, retry_count, retry_delay).await },
        timeout,
    )
    .await
    .map_err(|_| panic!("Test timed out"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::create_mock_interface;
    
    #[test]
    fn test_create_test_data() {
        let data = create_test_data(5);
        assert_eq!(data, vec![0, 1, 2, 3, 4]);
        assert!(verify_test_data(&data));
    }
    
    #[tokio::test]
    async fn test_run_with_retries() {
        let mut counter = 0;
        let result = run_with_retries(
            || {
                counter += 1;
                if counter < 3 {
                    Err("retry")
                } else {
                    Ok("success")
                }
            },
            3,
            Duration::from_millis(10),
        )
        .await;
        
        assert_eq!(result, Ok("success"));
        assert_eq!(counter, 3);
    }
    
    #[tokio::test]
    async fn test_run_with_timeout() {
        let result = run_with_timeout(
            async {
                time::sleep(Duration::from_millis(50)).await;
                "success"
            },
            Duration::from_millis(100),
        )
        .await;
        
        assert_eq!(result, Ok("success"));
    }
    
    #[tokio::test]
    async fn test_run_with_timeout_timeout() {
        let result = run_with_timeout(
            async {
                time::sleep(Duration::from_millis(100)).await;
                "success"
            },
            Duration::from_millis(50),
        )
        .await;
        
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_test_context() {
        let mock = create_mock_interface();
        let context = TestContext::new(
            mock,
            Duration::from_millis(100),
            3,
            Duration::from_millis(10),
        );
        
        assert!(context.setup().await.is_ok());
        assert!(context.teardown().await.is_ok());
    }
} 