/// modes of operation
/// 
/// This module contains the modes of operation for the application.
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;
use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};

use crate::datastore::DataStore;
use crate::rpcclient::RpcClient;

#[derive(Debug)]
pub struct Mode {
    pub (crate) rpc: Option<RpcClient>,
    pub (crate) store: Option<DataStore>,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            rpc: Some(RpcClient::default()),
            store: Some(DataStore::default()),
        }
    }
}

impl Mode {
    pub async fn new(dbname: String) -> Self {
        let mut ds = DataStore {
            dbname: "bitcoin".to_string(),
            ..Default::default()
        };
        ds.connect().await.unwrap();
        Mode {
            store: Some(ds),
            ..Default::default()
        }
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