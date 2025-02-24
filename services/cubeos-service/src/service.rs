// services/cubeos-service/src/service.rs
//
// Copyright (C) 2022 CUAVA
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
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
//

use std::sync::Arc;
use cubeos_core::{error::{Error, Result}, ping::Ping};
use udp_rs::Udp;
use kubos_system::Config;

pub struct Service {
    config: Config,
    udp_handler: Option<Arc<dyn Fn(&Udp) -> Result<()>>>,
}

impl Service {
    pub fn new(config: Config, udp_handler: Option<Arc<dyn Fn(&Udp) -> Result<()>>>) -> Self {
        Service {
            config,
            udp_handler,
        }
    }

    pub fn start(&self) -> Result<()> {
        log::info!("Starting cubeos-service...");
        if let Some(handler) = &self.udp_handler {
            let udp = Udp::new(&self.config.hosturl().unwrap_or("127.0.0.1:8082".to_string()))?;
            handler(&udp)?;
        }
        Ok(())
    }
}

impl Ping for Service {
    fn ping(&self) -> Result<()> {
        log::info!("Pinging cubeos-service...");
        Ok(())
    }
}