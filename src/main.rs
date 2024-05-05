use bitcoincore_rpc::{bitcoin::Amount, Auth, Client, RpcApi};
use tokio::main;

#[main]
async fn main() {
    let rpc = Client::new(
        "http://localhost:8332", 
        Auth::UserPass(
            "YOURUSERNAME".to_string(), 
            "YOURPASSWORD".to_string()
        )
    ).unwrap();
    println!("{:?}", get_block_fees(&rpc).await.unwrap()); // Print the fees of the last 144 blocks
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