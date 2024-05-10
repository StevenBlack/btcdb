drop table blockstats;
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
    owner to steve;

grant delete, insert, select on public.blockstats to rpc;

