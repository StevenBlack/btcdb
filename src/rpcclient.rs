#![allow(dead_code)]
// use bitcoin::io::Error;
use bitcoincore_rpc::{Auth, Client};

use crate::config::{get_rpcconfig, RPCConfig};

#[derive(Debug)]
pub (crate) struct RpcClient {
    pub (crate) rpc: bitcoincore_rpc::Client,
}

impl RpcClient {
    pub async fn new() -> Self {
        let config: RPCConfig = get_rpcconfig();
        RpcClient {
            rpc: Client::new(
            config.url.as_str(),
            Auth::UserPass(
                config.username,
                config.password,
            )
            ).expect("\nFailed to create Bitcoin RPC client connection\nPlease check your RPC configuration in the config.toml file.\n\n"),
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
