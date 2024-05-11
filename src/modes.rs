/// modes of operation
/// 
/// This module contains the modes of operation for the application.
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;
use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};

#[derive(Debug)]
pub struct Mode {
    pub rpc: Option<Client>,
    pub store: Option<DataStore>,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            rpc: Some(Client::new(
                "http://localhost:8332", 
                Auth::UserPass(
                    "YOURUSERNAME".to_string(), 
                    "YOURPASSWORD".to_string()
                )
            ).unwrap()),
            store:Some(DataStore::default()),
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

#[derive(Debug)]
pub struct DataStore {
    client: Option<tokio_postgres::Client>,
    host: String,
    dbname: String,
    username: String,
    password: String,
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            client: None,
            dbname: "bitcoin".to_string(),
            host: "localhost".to_string(),
            username: "rpc".to_string(),
            password: "YOURPASSWORD".to_string(),
        }
    }
}

impl DataStore {
    pub async fn new() -> Self {
        let mut ds = DataStore {
            client: None,
            dbname: "bitcoin".to_string(),
            host: "localhost".to_string(),
            username: "rpc".to_string(),
            password: "YOURPASSWORD".to_string(),
        };
        ds.connect().await.unwrap();
        ds
    }

    pub async fn connect(&mut self) -> Result<(), tokio_postgres::Error> {
        let (client, _connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}", 
                self.host, 
                self.username, 
                self.password, 
                self.dbname
            )
            , NoTls)
            .await?;
        self.client = Some(client);
        Ok(())
    }
}


// does database exist?
//   select exists (select * from pg_database where datname = 'the_name');
// does table exist?
//  select exists (select * from information_schema.tables where table_name = 'the_name');