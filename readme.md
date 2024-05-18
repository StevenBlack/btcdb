# 🧰 btcdb

This repository is a syncronizing bridge between a local Bitcoin blockchain, and a PostgreSQL 
data store.

## Motivation

A Bitcoin blockhain is a [LevelDB](https://en.wikipedia.org/wiki/LevelDB) database wich offers 
very limited querying capability. [Bitcoin Core](https://bitcoin.org/en/bitcoin-core/) maintains 
the local blockchain, and provides an [extensive API named BitcoinRPC](https://developer.bitcoin.org/reference/rpc/), 
but offers no way to query and aggregate the blockchain directly.


I really want a way to explore the Bitcoin blockchain as a SQL database. PostgreSQL is used here 
because I use it all the time, it's awesome, and it's free.

## Current Capabilities

Get help with by using the `-h` or `--help` flag.

        $ btcdb -h
        Usage: btcdb [OPTIONS]

        Options:
        -r, --raise <RAISE>  raise the datastore height by an arbitrary nummber of blocks
        -s, --status         display the status of the datastore and the local blockchain
        -u, --update         update the local datastore with the latest block stats
        -v, --verbose        verbose output from commands
        -h, --help           Print help
        -V, --version        Print version

Presently this repo offers:

1. Mapping between the [getblockstats api endpoint](https://developer.bitcoin.org/reference/rpc/getblockstats.html)
and a PostgreSQL table named `blockstats` which contains a column for (almost) each field returned
by the `getblockstats` api call.
  * We can **raise** the PostgreSQL `blockstats` table by an arbitrary number of blocks:

        # raise the local PostgreSQL table by 100 blocks    
        $ btcdb -r 100

  * We can **update** the PostgreSQL `blockstats` to level with the height of the local blockchain
      
        # update the local PostgreSQL table to the tip of the local blockchain.  
        $ btcdb -u 


2. Display the **status** of the local `blockstats` table and the local copy of the blockchain.

        # display the status of the PostgreSQL table to the tip of the local blockchain.  
        $ btcdb -s
        blockchain height: 844006
        store height: 844003
        store is 3 blocks behind the local blockchain

3. Presently calling `btcdb` without flags or options returns the status of the store and the
local blockchain.

        $ btcdb
        blockchain height: 844006
        store height: 844003
        store is 3 blocks behind the local blockchain 


See 
* [Bitcoin Core RPC documentation](https://developer.bitcoin.org/reference/rpc/index.html)
* [Storing and Querying Bitcoin Blockchain Using SQL Databases](https://files.eric.ed.gov/fulltext/EJ1219543.pdf)
