/*
 * Test Runner for Hardware Interface Testing
 * Copyright (C) 2024
 */

use crate::{HardwareInterface, HardwareResult, InterfaceStatus};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fmt;

/// Test result status
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed(String),
    Skipped(String),
    Error(String),
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration: Duration,
    pub error_count: u32,
    pub warning_count: u32,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Test: {}\nStatus: {:?}\nDuration: {:?}\nErrors: {}\nWarnings: {}\n",
            self.name,
            self.status,
            self.duration,
            self.error_count,
            self.warning_count
        )
    }
}

/// Test suite result
#[derive(Debug, Clone)]
pub struct TestSuiteResult {
    pub name: String,
    pub results: Vec<TestResult>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub error_tests: usize,
    pub total_duration: Duration,
}

impl fmt::Display for TestSuiteResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Test Suite: {}\nTotal Tests: {}\nPassed: {}\nFailed: {}\nSkipped: {}\nErrors: {}\nTotal Duration: {:?}\n\nResults:\n",
            self.name,
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            self.skipped_tests,
            self.error_tests,
            self.total_duration
        )?;
        
        for result in &self.results {
            write!(f, "{}", result)?;
        }
        
        Ok(())
    }
}

/// Test runner
pub struct TestRunner<T: HardwareInterface> {
    interface: Arc<Mutex<T>>,
    timeout: Duration,
    retry_count: u32,
    retry_delay: Duration,
}

impl<T: HardwareInterface> TestRunner<T> {
    pub fn new(interface: T, timeout: Duration, retry_count: u32, retry_delay: Duration) -> Self {
        Self {
            interface: Arc::new(Mutex::new(interface)),
            timeout,
            retry_count,
            retry_delay,
        }
    }
    
    pub async fn run_test<F>(&self, name: &str, test_fn: F) -> TestResult
    where
        F: FnOnce(Arc<Mutex<T>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = HardwareResult<()>> + Send>>,
    {
        let start = Instant::now();
        let mut error_count = 0;
        let mut warning_count = 0;
        
        let result = match test_fn(self.interface.clone()).await {
            Ok(_) => {
                let status = self.interface.lock().await.get_status().await;
                match status {
                    Ok(status) => {
                        error_count = status.error_count;
                        warning_count = status.warning_count;
                        if status.error_count == 0 {
                            TestStatus::Passed
                        } else {
                            TestStatus::Failed(format!("{} errors reported", status.error_count))
                        }
                    }
                    Err(e) => TestStatus::Error(format!("Failed to get status: {:?}", e)),
                }
            }
            Err(e) => TestStatus::Error(format!("Test failed: {:?}", e)),
        };
        
        TestResult {
            name: name.to_string(),
            status: result,
            duration: start.elapsed(),
            error_count,
            warning_count,
        }
    }
    
    pub async fn run_test_suite<F>(&self, name: &str, tests: Vec<(&str, F)>) -> TestSuiteResult
    where
        F: FnOnce(Arc<Mutex<T>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = HardwareResult<()>> + Send>>,
    {
        let start = Instant::now();
        let mut results = Vec::new();
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut skipped_tests = 0;
        let mut error_tests = 0;
        
        for (test_name, test_fn) in tests {
            let result = self.run_test(test_name, test_fn).await;
            
            match result.status {
                TestStatus::Passed => passed_tests += 1,
                TestStatus::Failed(_) => failed_tests += 1,
                TestStatus::Skipped(_) => skipped_tests += 1,
                TestStatus::Error(_) => error_tests += 1,
            }
            
            results.push(result);
        }
        
        TestSuiteResult {
            name: name.to_string(),
            results,
            total_tests: tests.len(),
            passed_tests,
            failed_tests,
            skipped_tests,
            error_tests,
            total_duration: start.elapsed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::create_mock_interface;
    
    #[tokio::test]
    async fn test_run_test() {
        let mock = create_mock_interface();
        let runner = TestRunner::new(
            mock,
            Duration::from_millis(100),
            3,
            Duration::from_millis(10),
        );
        
        let result = runner
            .run_test("test_initialize", |interface| {
                Box::pin(async move {
                    let mut interface = interface.lock().await;
                    interface.initialize().await
                })
            })
            .await;
        
        assert_eq!(result.status, TestStatus::Passed);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.warning_count, 0);
    }
    
    #[tokio::test]
    async fn test_run_test_suite() {
        let mock = create_mock_interface();
        let runner = TestRunner::new(
            mock,
            Duration::from_millis(100),
            3,
            Duration::from_millis(10),
        );
        
        let tests = vec![
            (
                "test_initialize",
                |interface: Arc<Mutex<_>>| {
                    Box::pin(async move {
                        let mut interface = interface.lock().await;
                        interface.initialize().await
                    })
                },
            ),
            (
                "test_deinitialize",
                |interface: Arc<Mutex<_>>| {
                    Box::pin(async move {
                        let mut interface = interface.lock().await;
                        interface.deinitialize().await
                    })
                },
            ),
        ];
        
        let result = runner.run_test_suite("test_suite", tests).await;
        
        assert_eq!(result.total_tests, 2);
        assert_eq!(result.passed_tests, 2);
        assert_eq!(result.failed_tests, 0);
        assert_eq!(result.skipped_tests, 0);
        assert_eq!(result.error_tests, 0);
    }
} 