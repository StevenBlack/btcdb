#![allow(dead_code)]
// use bitcoin::io::Error;
use bitcoincore_rpc::{Auth, Client};

// #[derive(Debug)]

#[derive(Debug)]
pub (crate) struct RpcClient {
    pub (crate) rpc: bitcoincore_rpc::Client,
}

impl RpcClient {
    pub async fn new() -> Self {
        RpcClient {
            rpc: Client::new(
            "http://localhost:8332", 
            Auth::UserPass(
                "YOURUSERNAME".to_string(), 
                "YOURPASSWORD".to_string()
            )
            ).unwrap(),
        }
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        RpcClient {
            rpc: Client::new(
            "http://localhost:8332", 
            Auth::UserPass(
                "YOURUSERNAME".to_string(), 
                "YOURPASSWORD".to_string()
            )
            ).unwrap(),
        }
    }
}