#![allow(dead_code)]


use crate::config::get_sqlconfig;
use crate::datastore::DataStore;
use crate::rpcclient::RpcClient;

#[derive(Debug)]
pub struct Mode {
    pub (crate) rpc: RpcClient,
    pub (crate) store: DataStore,
}

impl Mode {
    pub async fn new() -> Self {
        let sqlconfig = get_sqlconfig();
        let db_client = DataStore::new(sqlconfig).await;
        Mode {
            rpc: RpcClient::default(),
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

// let conf1 = MyConfiguration {
//     check: true,
//     ..Default::default()
// };



// does database exist?
//   select exists (select * from pg_database where datname = 'the_name');
// does table exist?
//  select exists (select * from information_schema.tables where table_name = 'the_name');