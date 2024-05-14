use tokio_postgres::{tls::NoTlsStream, Connection, Error, NoTls};
use bitcoincore_rpc::json::GetBlockStatsResult;

#[derive(Debug)]
pub struct DataStore {
    pub(crate) client: Option<tokio_postgres::Client>,
    pub(crate) host: String,
    pub(crate) dbname: String,
    pub(crate) schema: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            client: None,
            dbname: "bitcoin".to_string(),
            schema: "public".to_string(),
            host: "localhost".to_string(),
            username: "rpc".to_string(),
            password: "YOURPASSWORD".to_string(),
        }
    }
}

impl DataStore {
    pub async fn new() -> Self {
        let mut ds = DataStore {
            client: None,
            dbname: "bitcoin".to_string(),
            schema: "public".to_string(),
            host: "localhost".to_string(),
            username: "rpc".to_string(),
            password: "YOURPASSWORD".to_string(),
        };
        ds.connect().await.unwrap();
        ds
    }

    pub async fn connect(&mut self) -> Result<(), tokio_postgres::Error> {
        let (client, _connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}", 
                self.host, 
                self.username, 
                self.password, 
                self.dbname
            )
            , NoTls)
            .await?;
        self.createschemaenvironment().await?;
        self.client = Some(client);
        Ok(())
    }

    pub async fn createschemaenvironment(&self) -> Result<(), tokio_postgres::Error> {
        let client = self.client.as_ref().unwrap();
        client.batch_execute(self.initsql().as_str()).await?;
        Ok(())
    }

    /// ensure the requisite SQL schema and tables are in place
    fn initsql(&self) -> String {
        format!(
        "create schema if not exists {};
        set search_path to {}, public;
        create table if not exists public.blockstats
        (
            height         bigint         not null,
            blockhash      text           not null,
            avgfee         bigint         not null,
            avgfeerate     bigint         not null,
            avgtxsize      bigint         not null,
            ins            bigint         not null,
            outs           bigint         not null,
            subsidy        float          not null,
            swtotal_size   bigint         not null,
            swtotal_weight bigint         not null,
            swtxs          bigint         not null,
            time           bigint         not null,
            total_out      bigint         not null,
            total_size     bigint         not null,
            total_weight   bigint         not null,
            totalfee       float          not null,
            txs            bigint         not null,
            utxo_increase  bigint         not null,
            utxo_size_inc  bigint         not null,
            maxfee         bigint         not null,
            maxfeerate     bigint         not null,
            maxtxsize      bigint         not null,
            medianfee      bigint         not null,
            mediantime     bigint         not null,
            mediantxsize   bigint         not null,
            minfee         bigint         not null,
            minfeerate     bigint         not null,
            mintxsize      bigint         not null
        );
        
        alter table public.blockstats
            owner to {};
        
        grant delete, insert, select on public.blockstats to {};
        "
        , &self.schema
        , &self.schema
        , "steve"
        , "rpc"
    )
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