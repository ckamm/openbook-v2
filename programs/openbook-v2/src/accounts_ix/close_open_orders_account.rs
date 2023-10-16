use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseOpenOrdersAccount<'info> {
    // AUDIT: isn't it confusing that the lamports for the closed open orders go to sol_destination
    // while the lamports for reducing open_orders_indexer size go to payer?
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"OpenOrdersIndexer".as_ref(), owner.key().as_ref()],
        bump = open_orders_indexer.bump,
        realloc = OpenOrdersIndexer::space(open_orders_indexer.addresses.len()-1),
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub open_orders_indexer: Account<'info, OpenOrdersIndexer>,

    #[account(
        mut,
        has_one = owner,
        close = sol_destination,
    )]
    pub open_orders_account: AccountLoader<'info, OpenOrdersAccount>,
    #[account(mut)]
    /// CHECK: target for account rent needs no checks
    pub sol_destination: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
