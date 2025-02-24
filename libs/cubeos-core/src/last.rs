// libs/cubeos-core/src/last.rs
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

use crate::error::Result;

/// Trait for handling the last command and error
pub trait Last {
    fn set_last_cmd(&mut self, input: Vec<u8>);
    fn get_last_cmd(&self) -> Result<Vec<u8>>;
    fn set_last_err(&mut self, err: crate::error::Error);
    fn get_last_err(&self) -> Result<crate::error::Error>;
}