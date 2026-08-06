use anchor_lang::prelude::*;
use crate::state::{TreasuryState, AssetRecord};

#[derive(Accounts)]
pub struct RecordAssetPurchase<'info> {
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, TreasuryState>,
    #[account(init, payer = authority, space = AssetRecord::SPACE, seeds = [b"asset", treasury.key().as_ref(), &(treasury.asset_backing_count as u32).to_le_bytes()], bump)]
    pub asset_record: Account<'info, AssetRecord>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
