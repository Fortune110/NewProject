// services/cubeos-app/src/main.rs
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

mod app;
mod app_macro;
mod power;

use cubeos_core::error::{Error, Result};
use app::{App, Power};

fn main() -> Result<()> {
    let app_func = || -> Result<()> {
        println!("App running...");
        Ok(())
    };

    let mut app = App::new(app_func, Some(0b111), "cubeos-service");
    app.run()
}