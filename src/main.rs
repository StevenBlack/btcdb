// use bitcoincore_rpc::{Auth, Client, RpcApi};
use tokio_postgres::Error;

use utils::*;

mod modes;
mod datastore;
mod rpcclient;
mod utils;

#[tokio::main]
async fn main()  -> Result<(), Error> {

    let mode = modes::Mode::new().await;

    let height = get_store_height(&mode).await.unwrap();
    dbg!(height);

    raise_blockstats_table(&mode, 50000).await.unwrap();

    let height = get_store_height(&mode).await.unwrap();
    dbg!(height);
    
    // let fees = get_block_fees(&mode, 2).await.unwrap();
    // dbg!(fees);
    
    Ok(())
}


