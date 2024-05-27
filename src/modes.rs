#![allow(dead_code)]

use crate::config::get_sqlconfig;
use crate::datastore::DataStore;
use crate::rpcclient::RpcClient;

#[derive(Debug)]
pub struct Mode {
    pub rpc: RpcClient,
    pub store: DataStore,
}

impl Mode {
    pub async fn new() -> Self {
        let db_client = DataStore::new(get_sqlconfig()).await;
        Mode {
            rpc: RpcClient::new().await,
            store: db_client,
        }
    }

    pub fn getrpc(self) -> bitcoincore_rpc::Client {
        self.rpc.rpc
    }

    pub fn getstoreclient(self) -> tokio_postgres::Client {
        self.store.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoincore_rpc::RpcApi;

    #[tokio::test]
    async fn test_mode_new() {
        let mode = Mode::new().await;
        assert!(mode.rpc.rpc.get_blockchain_info().is_ok());
        assert!(mode.store.client.query("SELECT 1", &[]).await.is_ok());
    }
}
