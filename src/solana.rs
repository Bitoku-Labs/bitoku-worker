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

use super::*;
use anyhow::{Ok, Result};
use solana_client::rpc_client::RpcClient;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiMessage, UiTransaction, UiTransactionEncoding,
};
use std::sync::Arc;

//configuration for  RPC client
pub async fn get_configs() -> Result<(
    RpcTransactionLogsFilter,
    RpcTransactionLogsConfig,
    PubsubClient,
)> {
    let filter = RpcTransactionLogsFilter::Mentions(vec![String::from(PROGRAM)]);

    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::processed()),
    };

    let pub_sub_client = PubsubClient::new(WEB_SOCKET_URL).await?;

    Ok((filter, config, pub_sub_client))
}

//Prcessing logs
pub async fn process_logs(log: Response<RpcLogsResponse>) -> Result<()> {
    let url = String::from(RPC_URL);
    let connection = Arc::new(RpcClient::new_with_commitment(
        url.clone(),
        CommitmentConfig::confirmed(),
    ));

    let string = String::from("Program log: Instruction : SendRequest");
    let program_log = &log.value.logs;

    if program_log.contains(&string) {
        //creating a new thread to process the request
        thread::spawn(move || {
            let connection_clone = Arc::clone(&connection);

            let sign = log.value.signature;

            println!("sleeping");
            //sleeping for 20 seconds to make sure the transaction is confirmed
            thread::sleep(Duration::from_secs(20));

            //getting the request data
            let signature = sign.clone();
            let req = get_req_acc(signature.as_str(), connection_clone.as_ref()).unwrap();
            let req_data = get_acc_data(req, connection_clone.as_ref()).unwrap();

            process_request(req_data).unwrap();
        });
    }
    Ok(())
}

pub fn get_acc_data(req: String, connection: &RpcClient) -> Result<Vec<u8>> {
    let pk = Pubkey::from_str(&req)?;

    let data = connection.get_account_data(&pk)?;

    Ok(data)
}

fn get_req_acc(signature: &str, connection: &RpcClient) -> Result<String> {
    
    let sig = Signature::from_str(signature)?;

    let transaction_data = connection.get_transaction(&sig, UiTransactionEncoding::JsonParsed)?;

    let req = decode_tx(transaction_data)?;

    Ok(req)
}

fn decode_tx(data: EncodedConfirmedTransactionWithStatusMeta) -> Result<String> {
    let iter = serde_json::to_value(data.transaction.transaction)?;

    let iter2: UiTransaction = serde_json::from_value(iter)?;

    let message: UiMessage = iter2.message;

    match message {
        UiMessage::Parsed(parsed_message) => Ok(parsed_message.account_keys[1].pubkey.to_string()),

        _ => Ok("null".to_string()),
    }
}
