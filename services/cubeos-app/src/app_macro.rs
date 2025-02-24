// services/cubeos-app/src/app_macro.rs
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

#[macro_export]
macro_rules! app_macro {
    (
        $app: tt: $timeout: tt;
        $service: tt: $struct: tt {
            $(
                $(query)?$(mutation)?: $type: ident => fn $func: tt (&$(mut )?self $(,$msg: tt: $cmd: ty)*) -> $ign1: tt<$rep: ty> $(; out: $gql_q: ty)?;
            )*
        }
    ) => {
        use cubeos_service::udp::Connection;
        use cubeos_service::command_id;
        use std::str::FromStr;
        use log::debug;

        command_id! {
            $($type,)*
        }        

        lazy_static! {
            static ref HOST_URL: String = {
                kubos_system::Config::new(&stringify!($service).replace("_", "-"))
                    .unwrap()
                    .hosturl()
                    .unwrap()                  
            };
        }

        pub struct $struct {}
        impl $struct {
            $(
                pub fn $func($($msg: $cmd),*) -> Result<$rep, cubeos_core::error::Error> {
                    let app_url = "0.0.0.0:0".to_string();
                    let connection = Connection::from_path(app_url, HOST_URL.to_string());
                    let mut command = Command::serialize(CommandID::$type, ($($msg),*))?;
                    debug!("Command: {:?}", command);
                    match connection.transfer_timeout(command, std::time::Duration::from_secs($timeout)) {
                        Ok(response) => {
                            debug!("Response: {:?}", response);
                            match Command::<CommandID, $rep>::parse(&response) {
                                Ok(c) => Ok(c.data),
                                Err(e) => Err(e.into()),
                            }
                        },
                        Err(e) => Err(e.into()),
                    }
                }
            )*
        }       
    }
}