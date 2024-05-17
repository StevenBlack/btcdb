use bitcoincore_rpc::{Auth, Client, RpcApi};
use tokio_postgres::{Error, NoTls};

use datastore::BlockStats;
use utils::get_store_height;

mod modes;
mod datastore;
mod rpcclient;
mod utils;

#[tokio::main]
async fn main()  -> Result<(), Error> {

    let mode = modes::Mode::new().await;
    let height = get_store_height(&mode).await.unwrap();
    dbg!(height);
    
    Ok(())
}


