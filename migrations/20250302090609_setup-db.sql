-- Add migration script here

-- Deposit events by block number and transaction index
CREATE TABLE erc20_deposit_events(
    -- Auto incrementing ID handles the case when multiple deposits happen in the
    -- same transaction
    id INTEGER PRIMARY KEY NOT NULL AUTOINCREMENT,
    trader BLOB NOT NULL,
    token BLOB NOT NULL,
    lots BLOB NOT NULL,
    block_number INTEGER NOT NULL,
    tx_index INTEGER NOT NULL,
    tx_hash BLOB NOT NULL
);
