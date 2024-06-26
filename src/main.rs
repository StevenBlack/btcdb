// Copyright 2024 Steven Black <s@sbc.io>,
//
// Licensed under the MIT license
// http://opensource.org/licenses/MIT
// This file may not be copied, modified, or distributed
// except according to those terms.
//
use clap::Parser;
use tokio_postgres::Error;

use utils::*;

mod config;
mod datastore;
mod modes;
mod rpcclient;
mod utils;

/// Defines our command line flags and options
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    /// display configuration information.
    #[clap(short, long)]
    config: bool,

    /// raise the datastore height by an arbitrary nummber of blocks.
    #[clap(short, long)]
    raise: Option<u64>,

    /// display the status of the datastore and the local blockchain.
    #[clap(short, long)]
    status: bool,

    /// update the local datastore level with the local Bitcoin blockchain.
    #[clap(short, long)]
    update: bool,

    // /// verbose output from commands.
    // #[clap(short, long)]
    // verbose: bool,
}

#[tokio::main]
async fn main()  -> Result<(), Error> {

    // parce the command line arguments
    let cli = Cli::parse();

    // our mode of operation
    let mode = modes::Mode::new().await;

    if cli.config {
        config();
        return Ok(());
    }

    if cli.status {
        status().await;
        return Ok(());
    }

    if cli.raise.is_some() {
        let raise = cli.raise.unwrap();
        raise_blockstats_table(&mode, raise).await.unwrap();
        return Ok(());
    }

    if cli.update {
        update_blockstats_table(&mode).await.unwrap();
        return Ok(());
    }

    // if we get here, we have no command line arguments
    // so we just print the status
    status().await;

    Ok(())
}

/// Print the status of the local blockchain and our local data store.
async fn status() {
    let mode = modes::Mode::new().await;
    let blockchainheight = get_blockchain_height(&mode).await.unwrap();
    println!("blockchain height: {}", blockchainheight);
    let storeheight = get_store_height(&mode).await.unwrap();
    println!("store height: {}", storeheight);
    let delta = blockchainheight - storeheight;
    if delta > 0 {
        println!("store is {} blocks behind the local blockchain", blockchainheight - storeheight);
        return;
    }
    println!("store and the local blockchain have equal height");
}

/// Display configuration information.
/// * Location of the configuration file
/// * SQL configuration
/// * RPC configuration
fn config() {
    println!();
    println!("================================");
    println!("btcdb Configuration information:");
    println!("================================");
    println!();
    println!("btcdb configuration file lication:");
    println!("  {:?}", config::get_config_file());
    println!();
    println!("{}", config::get_sqlconfig());
    println!();
    println!("{}", config::get_rpcconfig());
    println!();
}

