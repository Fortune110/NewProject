//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//!
//! Common structure and functions for setting up service logging
//!

use log::LevelFilter;
use failure::*;
use simplelog::*;

/// Initialise logging for the service
/// 
/// In production environment (--release) all messages will be routed to syslog, DEBUG level messages will be discarded
/// 
/// In development environment all messages will be routed to the terminal and DEBUG level messages will be displayed
/// 
pub fn init() -> Result<(),Error> {
    let logger_type = std::env::var("RUST_LOGGER").unwrap_or_else(|_| String::from("syslog"));
    let log_level = match std::env::var("RUST_LOG") {
        Ok(val) => match val.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        },
        Err(_) => LevelFilter::Info,
    };

    if logger_type == "term" {
        match CombinedLogger::init(vec![
            TermLogger::new(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        ]) {
            Ok(()) => Ok(()),
            Err(e) => Err(format_err!("Failed to init terminal logger:{}",e)),
        }
    } else {        
        match syslog::init(syslog::Facility::LOG_DAEMON, log_level, None) {
            Ok(()) => Ok(()),
            Err(e) => Err(format_err!("Failed to init syslog:{}",e)),
        }
    }
}