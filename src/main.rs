use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::GetBlockStatsResult;
use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};
use tokio::{main, task::futures};

mod modes;

#[tokio::main]
async fn main()  -> Result<(), Error> {

    // let mut db = modes::DataStore::default();
    // db.connect().await.unwrap();
    // dbg!(db);



    let mode = modes::Mode::new("bitcoin".to_string()).await;
    dbg!(mode);

    let mode = modes::Mode{rpc: None, store: None};
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



pub struct BlockStats {
    pub height: u64,
    pub blockhash: String,
    pub avgfee: u64,
    pub avgfeerate: u64,
    pub avgtxsize: u32,
    pub ins: u64,
    pub outs: u64,
    pub subsidy: f64,
    pub swtotal_size: u64,
    pub swtotal_weight: u64,
    pub swtxs: u64,
    pub time: u64,
    pub total_out: u64,
    pub total_size: u64,
    pub total_weight: u64,
    pub totalfee: f64,
    pub txs: u64,
    pub utxo_increase: u64,
    pub utxo_size_inc: u64,
    pub maxfee: u64,
    pub maxfeerate: u64,
    pub maxtxsize: u64,
    pub medianfee: u64,
    pub mediantime: u64,
    pub mediantxsize: u64,
    pub minfee: u64,
    pub minfeerate: u64,
    pub mintxsize: u64,
}

impl BlockStats {
    pub fn from_rpc(stats: GetBlockStatsResult) -> Self {

        Self {
            avgfee: stats.avg_fee.to_sat(),
            avgfeerate: stats.avg_fee_rate.to_sat(),
            avgtxsize: stats.avg_tx_size,
            blockhash: stats.block_hash.to_string(),
            height: stats.height,
            ins: stats.ins as u64,
            maxfee: stats.max_fee.to_sat(),
            maxfeerate: stats.max_fee_rate.to_sat() as u64,
            maxtxsize: stats.max_tx_size as u64,
            medianfee: stats.median_fee.to_sat(),
            mediantime: stats.median_time,
            mediantxsize: stats.median_tx_size as u64,
            minfee: stats.min_fee.to_sat(),
            minfeerate: stats.min_fee_rate.to_sat() as u64,
            mintxsize: stats.min_tx_size as u64,
            outs: stats.outs as u64,
            subsidy: stats.subsidy.to_btc() as f64,
            swtotal_size: stats.sw_total_size as u64,
            swtotal_weight: stats.sw_total_weight as u64,
            swtxs: stats.sw_txs as u64,
            time: stats.time,
            total_out: stats.total_out.to_sat(),
            total_size: stats.total_size as u64,
            total_weight: stats.total_weight as u64,
            totalfee: stats.total_fee.to_btc() as f64,
            txs: stats.txs as u64,
            // utxo_increase_actual: stats.utxo_increase_actual,
            utxo_increase: stats.utxo_increase as u64,
            // utxo_size_inc_actual: stats.utxo_size_inc_actual,
            utxo_size_inc: stats.utxo_size_inc as u64,
        }
    }

    pub async fn insert(&self, client: &tokio_postgres::Client) -> Result<(), Error> {
        client.execute(
            "INSERT INTO public.blockstats (
                height
                , blockhash
                , avgfee
                , avgfeerate
                , avgtxsize

                , ins
                , outs
                , subsidy
                , swtotal_size
                , swtotal_weight
                
                , swtxs
                , time
                , total_out
                , total_size
                , total_weight
                
                , totalfee
                , txs
                , utxo_increase
                , utxo_size_inc
                , maxfee
                
                , maxfeerate
                , maxtxsize
                , medianfee
                , mediantime
                , mediantxsize
                
                , minfee
                , minfeerate
                , mintxsize
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, 
                $19, $20, $21, $22, $23, $24, $25, $26, $27, $28
            )",
            &[
                &(self.height as i64)
                , &self.blockhash.to_string()
                , &(self.avgfee as i64)
                , &(self.avgfeerate as i64)
                , &(self.avgtxsize as i64)
                
                , &(self.ins as i64)
                , &(self.outs as i64)
                , &(self.subsidy as f64)
                , &(self.swtotal_size as i64)
                , &(self.swtotal_weight as i64)
                
                , &(self.swtxs as i64)
                , &(self.time as i64)
                , &(self.total_out as i64)
                , &(self.total_size as i64)
                , &(self.total_weight as i64)
                
                , &(self.totalfee as f64)
                , &(self.txs as i64)
                , &(self.utxo_increase as i64)
                , &(self.utxo_size_inc as i64)
                , &(self.maxfee as i64)
                
                , &(self.maxfeerate as i64)
                , &(self.maxtxsize as i64)
                , &(self.medianfee as i64)
                , &(self.mediantime as i64)
                , &(self.mediantxsize as i64)
                
                , &(self.minfee as i64)
                , &(self.minfeerate as i64)
                , &(self.mintxsize as i64)
            ]
        ).await?;

        Ok(())
    }
}