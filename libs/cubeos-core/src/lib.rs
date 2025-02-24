// CUBEOS_WORKSPACE/libs/cubeos-core/src/lib.rs
//
// Copyright (C) 2017 Kubos Corporation
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
// 2022 rewritten for CubeOS
//
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
//

mod command;
mod last;
mod ping;
mod error;

// 引用 sec 子 crate
pub use cubeos_core_sec::Security;

pub use crate::error::{Error, Result};
pub use crate::ping::Ping;
pub use crate::last::Last;
pub use crate::command::{Command, command_id};

pub use kubos_system::logger as Logger;
pub use kubos_system::Config;