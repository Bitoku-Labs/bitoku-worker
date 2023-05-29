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

use anyhow::Result;
use futures_util::StreamExt;
use std::str::FromStr;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::thread;
use std::time::Duration;

pub mod helper;
pub mod request;
pub mod solana;
pub mod worker;
use request::*;
use worker::*;
use helper::*;

#[tokio::main]
async fn main() -> Result<()> {
    worker().await?;

    Ok(())
}
