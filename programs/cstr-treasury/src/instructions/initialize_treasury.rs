use anchor_lang::prelude::*;
use crate::state::TreasuryState;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(init, payer = authority, space = TreasuryState::SPACE, seeds = [b"treasury"], bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
