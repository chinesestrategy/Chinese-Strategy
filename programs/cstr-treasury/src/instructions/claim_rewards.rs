use anchor_lang::prelude::*;
use crate::state::{TreasuryState, DistributionSnapshot, HolderClaim};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(mut, seeds = [b"snapshot", treasury.key().as_ref(), &snapshot.epoch.to_le_bytes()], bump = snapshot.bump)]
    pub snapshot: Account<'info, DistributionSnapshot>,
    #[account(init_if_needed, payer = holder, space = HolderClaim::SPACE, seeds = [b"claim", treasury.key().as_ref(), holder.key().as_ref(), &snapshot.epoch.to_le_bytes()], bump)]
    pub claim_state: Account<'info, HolderClaim>,
    #[account(mut)]
    pub holder: Signer<'info>,
    /// CHECK: recipient for claim payments
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
