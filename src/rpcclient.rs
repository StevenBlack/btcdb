use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;

#[derive(Debug)]

pub (crate) struct RpcClient {
    pub (crate) rpc: Option<Client>,
}

impl RpcClient {
    pub async fn new() -> Self {
        RpcClient {
            rpc: Some(Client::new(
                "http://localhost:8332", 
                Auth::UserPass(
                    "YOURUSERNAME".to_string(), 
                    "YOURPASSWORD".to_string()
                )
            ).unwrap()),
        }
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        RpcClient {
            rpc: Some(Client::new(
                "http://localhost:8332", 
                Auth::UserPass(
                    "YOURUSERNAME".to_string(), 
                    "YOURPASSWORD".to_string()
                )
            ).unwrap()),
        }
    }
}