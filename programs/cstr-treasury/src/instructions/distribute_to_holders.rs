use anchor_lang::prelude::*;
use crate::state::{TreasuryState, DistributionSnapshot};

#[derive(Accounts)]
pub struct DistributeToHolders<'info> {
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(init, payer = authority, space = DistributionSnapshot::SPACE, seeds = [b"snapshot", treasury.key().as_ref(), &(treasury.current_epoch + 1).to_le_bytes()], bump)]
    pub snapshot: Account<'info, DistributionSnapshot>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
