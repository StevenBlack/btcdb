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

**Take note**: You'll need a copy of Bitcoin Core running and addressable.

```
Usage: btcdb [OPTIONS]

Options:
  -c, --config         display configuration information
  -r, --raise <RAISE>  raise the datastore height by an arbitrary nummber of blocks
  -s, --status         display the status of the datastore and the local blockchain
  -u, --update         update the local datastore level with the local Bitcoin blockchain
  -h, --help           Print help
  -V, --version        Print version
```

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

## How fast is this bridge?

### Loading the `blockstats` table

TL;DR: the `blockstats` table updates at a rate of about 50 blocks per second on this hardware.

Let's try an test and see.  Let's add 1,000 blocks to the `blockstats` table.

**Hardware:**

OS: Fedora Linux 40  
Host: ThinkPad X1 Carbon Gen 11
CPU: 13th Gen Intel i7-1370P @ 5.20 GHz
Memory: 32 GiB

```
$ btcdb
blockchain height: 844175
store height: 843173
store is 1002 blocks behind the local blockchain


$ time btcdb -u

real    0m22.798s
user    0m0.308s
sys     0m0.361s
```

Which is about 50 blocks per second.

### Querying the `blockstats` table

How many records do we have?  Presently the `blockstats` table has no indexes.

```swl
> select max(height) from blockstats;
```
```
+--------+
| max    |
|--------|
| 844184 |
+--------+
SELECT 1
Time: 0.126s
```

How many blocks have been mined for each calendar year?

```sql
> select extract(year from to_timestamp(time)) as year,
 count(*)
 from blockstats
 group by 1
 order by 1 desc
 ```
```
+------+-------+
| year | count |
|------+-------|
| 2024 | 20363 |
| 2023 | 53999 |
| 2022 | 53188 |
| 2021 | 52690 |
| 2020 | 53227 |
| 2019 | 54232 |
| 2018 | 54491 |
| 2017 | 55931 |
| 2016 | 54854 |
| 2015 | 54308 |
| 2014 | 58862 |
| 2013 | 63439 |
| 2012 | 54540 |
| 2011 | 59615 |
| 2010 | 67928 |
| 2009 | 32518 |
+------+-------+
SELECT 16
Time: 0.628s
```
So this aggregate query over the whole `blockstats` table happens in subsecond time.

### See 
* [Bitcoin Core RPC documentation](https://developer.bitcoin.org/reference/rpc/index.html)
* [Storing and Querying Bitcoin Blockchain Using SQL Databases](https://files.eric.ed.gov/fulltext/EJ1219543.pdf)
