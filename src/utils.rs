use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc_json::{GetBlockResult};
use tokio_postgres::{Error, NoTls};
use crate::{datastore::BlockStats, modes::Mode};


pub async fn get_store_height(moderef: &Mode) -> Result<i64, Error> {
    let row = moderef.store.client.query(
        "SELECT max(height) FROM public.blockstats;",
        &[]
    ).await.expect("Error getting max height");
    Ok(row[0].get::<_, i64>(0))
}


pub async fn get_block_fees(mode: Mode) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let rpc: bitcoincore_rpc::Client = mode.getrpc();
    let mut fees: Vec<u64> = Vec::new();
    let current_height = rpc.get_block_count().unwrap();

    for height in (current_height - 10)..=current_height {
        let block_hash = rpc.get_block_hash(height).unwrap();
        let block = rpc.get_block(&block_hash).unwrap();

        let total_reward: u64 = block.txdata[0].output.iter().map(|o| o.value.to_sat()).sum();
        let block_reward = get_block_reward(height);

        let fee = total_reward - block_reward;
        fees.push(fee);
    }

    Ok(fees)
}

pub fn get_block_reward(block_height: u64) -> u64 {
    let initial_reward = 50 * 100_000_000; // 50 BTC in satoshis
    let halvings = block_height / 210_000;
    let block_reward = initial_reward >> halvings;
    block_reward
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
    use crate::modes;

    use super::*;

    #[tokio::test]
    async fn test_get_store_height() {
        let mode = modes::Mode::new().await;
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
    
        // fetch an out of bounds block
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
    
