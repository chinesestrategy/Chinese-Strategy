use anchor_lang::prelude::*;
use crate::state::TreasuryState;

#[derive(Accounts)]
pub struct DepositFees<'info> {
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
