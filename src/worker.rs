/*
  Copyright 2023 Bitoku Labs

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::solana::{get_configs, process_logs};

use super::*;

pub async fn worker() -> Result<()> {
    println!("worker is listening ");

    let (filter, config, client) = get_configs().await?;

    //listening Program logs
    let (mut logs, logs2) = client.logs_subscribe(filter, config).await?;

    while let Some(log) = logs.next().await {
        //Processing program logs
        process_logs(log).await?;
    }
    logs2().await;

    Ok(())
}
