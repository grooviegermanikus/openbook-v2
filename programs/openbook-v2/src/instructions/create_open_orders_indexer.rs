use crate::accounts_ix::CreateOpenOrdersIndexer;
use anchor_lang::prelude::*;

// should be an implicit not explicit operation, or idempotent like associated token acount

// if you create and close a lot of accounts (e.g. mango users do that a lot due to OO limit)
// you will easily end up with created > 128 and created - closed < 4
// if you just trade every token once. if you frequently switch the number is unbounded
pub fn create_open_orders_indexer(ctx: Context<CreateOpenOrdersIndexer>) -> Result<()> {
    let mut indexer = ctx.accounts.open_orders_indexer.load_init()?;

    indexer.owner = ctx.accounts.owner.key();
    indexer.market = ctx.accounts.market.key();
    indexer.bump = *ctx.bumps.get("open_orders_indexer").unwrap();

    indexer.created_counter = 0;
    indexer.closed_counter = 0;

    Ok(())
}
