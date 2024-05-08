use bitcoincore_rpc::{bitcoin::Amount, Auth, Client, RpcApi};
use tokio::{main, task::futures};

#[tokio::main]
async fn main()  -> Result<(), Error> {
    let rpc_result = connect_to_bitcoin_core().await;
    match rpc_result {
        Ok(..) => {
            println!("Connected to Bitcoin Core!");
            // println!("{:?}", get_block_fees(&rpc).await.unwrap()); // Print the fees of the last 144 blocks
        },
        _ => {
            println!("Failed to connect to Bitcoin Core!");     
        }
    }

    connect_database().await.unwrap();
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

use tokio_postgres::{NoTls, Error};
async fn connect_database() -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=rpc password=YOURPASSWORD dbname=bitcoin", NoTls).await?;

    println!("Connected to the database!");

    if 1 == 1 {
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Prepare the INSERT statement
        let stmt = client.prepare("
            INSERT INTO blockstats (
                height, blockhash, avgfee, avgfeerate, avgtxsize, ins, outs, subsidy, swtotal_size, swtotal_weight, swtxs, time, total_out, total_size, total_weight, totalfee, txs, utxo_increase, utxo_size_inc, utxo_increase_actual, utxo_size_inc_actual, maxfee, maxfeerate, maxtxsize, medianfee, mediantime, mediantxsize, minfee, minfeerate, mintxsize
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30
            )
        ").await?;

        // Execute the INSERT statement
        client.execute(&stmt, &[
        //  height,      blockhash,                                                          avgfee,   avgfeerate, avgtxsize, ins, outs,     subsidy,       swtotal_size, swtotal_weight, swtxs, time,          total_out,       total_size,  total_weight, totalfee,     txs,      utxo_increase, utxo_size_inc, utxo_increase_actual, utxo_size_inc_actual, maxfee,     maxfeerate, maxtxsize, medianfee, mediantime,     mediantxsize, minfee,   minfeerate, mintxsize      
            &842209i32, &"000000000000000000023bc3e5419ae1a8d508f531c205e7b6493732d6948c51", &6461i32, &30i32, &337i32, &7791i32, &11689i32, &312500000i32, &1518730i64, &3782590i64, &4569i32, &1714927941i64, &22825097521i64, &1571081i64, &3991994i64,  &30085139i64, &4657i32, &3898i32,      &246275i64,    &476i32,              &41710i64,            &825190i64, &720i32,    &65795i32, &3483i32,  &1714923990i64, &182i32,      &1158i32, &3i32,      &150i32
        ]).await?;
    }
    Ok(())
}

async fn get_block_fees(rpc: &bitcoincore_rpc::Client) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut fees = vec![];
    let current_height = rpc.get_block_count()?;

    for height in (current_height - 144)..=current_height {
        let block  = rpc.get_block_stats(height).unwrap();
        let fee: f64 = block.total_fee.to_btc();
        fees.push(fee);
    }

    Ok(fees)
}

fn get_block_reward(block_height: u64) -> u64 {
    let initial_reward = 50 * 100_000_000; // 50 BTC in satoshis
    let halvings = block_height / 210_000;
    let block_reward = initial_reward >> halvings;
    block_reward
}

#[cfg(test)]
mod tests {
    use super::*;

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
}