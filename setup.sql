DROP TABLE public.blockstats;

CREATE TABLE public.blockstats (
    height INTEGER NOT NULL,
    blockhash TEXT NOT NULL,
    avgfee BIGINT NOT NULL,
    avgfeerate BIGINT NOT NULL,
    avgtxsize BIGINT NOT NULL,
    ins BIGINT NOT NULL,
    outs BIGINT NOT NULL,
    subsidy BIGINT NOT NULL,
    swtotal_size BIGINT NOT NULL,
    swtotal_weight BIGINT NOT NULL,
    swtxs BIGINT NOT NULL,
    time BIGINT NOT NULL,
    total_out BIGINT NOT NULL,
    total_size BIGINT NOT NULL,
    total_weight BIGINT NOT NULL,
    totalfee BIGINT NOT NULL,
    txs BIGINT NOT NULL,
    utxo_increase BIGINT NOT NULL,
    utxo_size_inc BIGINT NOT NULL,
    -- utxo_increase_actual BIGINT NOT NULL,
    -- utxo_size_inc_actual BIGINT NOT NULL,
    maxfee BIGINT NOT NULL,
    maxfeerate BIGINT NOT NULL,
    maxtxsize BIGINT NOT NULL,
    medianfee BIGINT NOT NULL,
    mediantime BIGINT NOT NULL,
    mediantxsize BIGINT NOT NULL,
    minfee BIGINT NOT NULL,
    minfeerate BIGINT NOT NULL,
    mintxsize BIGINT NOT NULL
);

alter table public.blockstats
    owner to steve;

grant delete, insert, select on public.blockstats to rpc;

