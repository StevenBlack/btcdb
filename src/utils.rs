#![allow(dead_code)]
use bitcoincore_rpc::{Auth, Client, RpcApi};
// use bitcoincore_rpc_json::{GetBlockResult};
use tokio_postgres::{Error, NoTls};
use crate::{datastore::BlockStats, modes::Mode};

/// returns the block height of the store
pub async fn get_store_height(moderef: &Mode) -> Result<u64, Error> {
    let row = moderef.store.client.query(
        "SELECT max(height) FROM public.blockstats;",
        &[]
    ).await.expect("Error getting max height");
    Ok(row[0].get::<_, i64>(0).try_into().unwrap())
}

/// returns the block height of the blockchain
pub async fn get_blockchain_height(moderef: &Mode) -> Result<u64, Box<dyn std::error::Error>> {
    let rpc: &Client = &moderef.rpc.rpc;
    Ok(rpc.get_block_count().unwrap())
}

/// a demo utility function to get the block fees for the last `numblocks` blocks
pub async fn get_block_fees(moderef: &Mode, numblocks: u64) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let rpc: &Client = &moderef.rpc.rpc;
    let mut fees: Vec<u64> = Vec::new();
    let current_height = rpc.get_block_count().unwrap();

    for height in (current_height - numblocks)..=current_height {
        let block_hash = rpc.get_block_hash(height).unwrap();
        let block = rpc.get_block(&block_hash).unwrap();

        let total_reward: u64 = block.txdata[0].output.iter().map(|o| o.value.to_sat()).sum();
        let block_reward = get_block_reward(height);

        let fee = total_reward - block_reward;
        fees.push(fee);
    }

    Ok(fees)
}

/// get the block reward for a given block height
pub fn get_block_reward(block_height: u64) -> u64 {
    let initial_reward = 50 * 100_000_000; // 50 BTC in satoshis
    let halvings = block_height / 210_000;
    let block_reward = initial_reward >> halvings;
    block_reward
}

/// update the blockstats table in the database, all the way to the current block
pub async fn update_blockstats_table(moderef: &Mode) -> Result<(), Error> {
    let rpc = &moderef.rpc.rpc;
    let client = &moderef.store.client;
    let current_height = rpc.get_block_count().unwrap();
    let store_height = get_store_height(&moderef).await.unwrap();
    for i in (store_height + 1)..=current_height {

        let block  = rpc.get_block_stats(i).unwrap();
        let block_stats = BlockStats::from_rpc(block);
        block_stats.insert(&client).await.unwrap();
        // println!("Inserted block {}", i);
    }    
    Ok(())
}

/// update the blockstats table in the database, all the way to the current block
pub async fn raise_blockstats_table(moderef: &Mode, numblocks: u64) -> Result<(), Error> {
    use std::cmp::min;
    let rpc = &moderef.rpc.rpc;
    let client = &moderef.store.client;
    let current_height = rpc.get_block_count().unwrap();
    let store_height = get_store_height(&moderef).await.unwrap();
    let increment = min(numblocks, current_height - store_height);
    for i in (store_height + 1)..= (store_height + increment) {
        let block  = rpc.get_block_stats(i).unwrap();
        let block_stats = BlockStats::from_rpc(block);
        block_stats.insert(&client).await.unwrap();
        // println!("Inserted block {}", i);
    }    
    Ok(())
}

async fn connect_to_bitcoin_core() -> Result<Client, bitcoincore_rpc::Error> {
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
    use super::*;

    #[tokio::test]
    async fn test_get_store_height() {
        let mode = Mode::new().await;
        let height = get_store_height(&mode).await.unwrap();
        assert!(height > 0);
    }


    #[test]
    fn test_get_block_reward_initial() {
        let block_reward = get_block_reward(0);
        assert_eq!(block_reward, 50 * 100_000_000);
    }

    #[test]
    fn test_get_block_reward_after_first_halving() {
        let block_reward = get_block_reward(210_000);
        assert_eq!(block_reward, 25 * 100_000_000);
    }

    #[test]
    fn test_get_block_reward_after_second_halving() {
        let block_reward = get_block_reward(420_000);
        assert_eq!(block_reward, 1_250_000_000);
    }

    #[test]
    fn test_get_block_reward_no_more_halvings() {
        let block_reward = get_block_reward(210_000 * 33); // After 33 halvings, reward should be 0
        assert_eq!(block_reward, 0);
    }

    #[cfg(test)]
    mod tests {    
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
    
        // fetch an out-of-bounds block
        #[tokio::test]
        async fn test_get_block_stats_out_of_bounds() {
            let rpc = connect_to_bitcoin_core().await;
            assert!(rpc.is_ok());
            let rpc = rpc.unwrap();
            let block = rpc.get_block_stats(999_999_999);
            assert!(block.is_err());
        }
        }
    }
    
