/// modes of operation
/// 
/// This module contains the modes of operation for the application.
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;
use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};

use crate::datastore::DataStore;
use crate::rpcclient::{self, RpcClient};

#[derive(Debug)]
pub struct Mode {
    pub (crate) rpc: RpcClient,
    pub (crate) store: DataStore,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            rpc: RpcClient::default(),
            store: DataStore::new(),
        }
    }
}

impl Mode {
    pub async fn new() -> Self {
        let mut db_client = DataStore::default();
        db_client.connect().await.unwrap();
        Mode {
            rpc: RpcClient::default(),
            store: db_client,
        }
    }

    pub fn getrpc(self) -> bitcoincore_rpc::Client {
        self.rpc.rpc
    }
    pub fn getstoreclient(self) -> tokio_postgres::Client {
        self.store.client.unwrap()
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