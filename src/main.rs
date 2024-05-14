use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;

use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};
use tokio::{main, task::futures};

use datastore::BlockStats;
use rpcclient::RpcClient;

mod modes;
mod datastore;
mod rpcclient;

#[tokio::main]
async fn main()  -> Result<(), Error> {

    // let mut db = modes::DataStore::default();
    // db.connect().await.unwrap();
    // dbg!(db);



    // let mode = modes::Mode::new("bitcoin".to_string()).await;
    // dbg!(mode);

    let mode = modes::Mode::default();
    dbg!(mode);

    // let db = modes::DataStore::new().await;
    // dbg!(db);



    // let rpc_connect_result = connect_to_bitcoin_core().await;
    // match rpc_connect_result {
    //     Ok(..) => {
    //         println!("Connected to Bitcoin Core!");
    //         // println!("{:?}", get_block_fees(&rpc).await.unwrap()); // Print the fees of the last 144 blocks
    //     },
    //     _ => {
    //         println!("Failed to connect to Bitcoin Core!");     
    //     }
    // }
    // let rpc = rpc_connect_result.unwrap();
    // let client: tokio_postgres::Client = connect_to_database().await.unwrap();





    // let block  = rpc.get_block_stats(840_000).unwrap();
    // let block_stats = BlockStats::from_rpc(block);
    // block_stats.insert(client).await.unwrap();


    
    Ok(())
}

async fn update_blockstats_table(rpc: &Client, client: &tokio_postgres::Client, blockstart: u64, blockend: u64) -> Result<(), Error> {
    for i in blockstart..=blockend {
        let block  = rpc.get_block_stats(i).unwrap();
        let block_stats = BlockStats::from_rpc(block);
        block_stats.insert(&client).await.unwrap();
        // println!("Inserted block {}", i);
    }    
    Ok(())
}

async fn connect_to_bitcoin_core() -> Result<bitcoincore_rpc::Client, bitcoincore_rpc::Error> {
    Client::new(
        "http://localhost:8332", 
        Auth::UserPass(
            "YOURUSERNAME".to_string(), 
            "YOURPASSWORD".to_string()
        )
    )
}

async fn connect_to_database() -> Result<tokio_postgres::Client, Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=rpc password=YOURPASSWORD dbname=bitcoin", NoTls).await?;

    println!("Connected to the database!");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = &connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

#[cfg(test)]
mod tests {
    use crate::datastore::DataStore;

    use super::*;

    // test connecting to the bitcoin core
    #[tokio::test]
    async fn test_connect_to_btc_core() {
        let rpc = connect_to_bitcoin_core().await;
        assert!(rpc.is_ok());
    }

    // test connecting to the database
    #[tokio::test]
    async fn test_connect_to_database() {
        let db = connect_to_database().await;
        assert!(db.is_ok());
    }

    // fetch a block.
    #[tokio::test]
    async fn test_get_block_stats() {
        let rpc = connect_to_bitcoin_core().await;
        assert!(rpc.is_ok());
        let rpc = rpc.unwrap();
        let block = rpc.get_block_stats(842209).unwrap();
        assert_eq!(block.height, 842209);
        println!("{:?}", block)
    }

    // fetch an out of bounds block
    #[tokio::test]
    async fn test_get_block_stats_out_of_bounds() {
        let rpc = connect_to_bitcoin_core().await;
        assert!(rpc.is_ok());
        let rpc = rpc.unwrap();
        let block = rpc.get_block_stats(999_999_999);
        assert!(block.is_err());
    }

    // thest the default mode
    #[tokio::test]
    async fn test_default_mode() {
        let mode = modes::Mode::default();
        assert!(mode.rpc.is_some());
        assert!(mode.store.is_some());
    }

    #[tokio::test]
    async fn test_empty_mode() {

    let mode = modes::Mode{
        rpc: None, 
        store: None
    };           
    assert!(mode.rpc.is_none());
    assert!(mode.store.is_none());
    }


    #[tokio::test]
    async fn test_explicitly_specified_mode() {
        let mode = modes::Mode{
            rpc: None, 
            store: Some(DataStore::default()),
        };           
        assert!(mode.rpc.is_none());
        assert!(mode.store.is_some());

        let mode = modes::Mode{
            rpc: Some(RpcClient::default()), 
            store: None,
        };           
        assert!(mode.rpc.is_some());
        assert!(mode.store.is_none());
    }
}
