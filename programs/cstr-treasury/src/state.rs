use anchor_lang::prelude::*;

#[account]
pub struct TreasuryState {
    pub authority: Pubkey,
    pub bump: u8,
    pub fee_bps: u16,
    pub paused: bool,
    pub total_sol_inflow: u64,
    pub total_usdc_inflow: u64,
    pub total_distributed_sol: u64,
    pub total_distributed_usdc: u64,
    pub current_epoch: u64,
    pub last_snapshot_slot: u64,
    pub asset_backing_count: u32,
    pub created_at: i64,
    pub reserved: [u8; 64],
}

impl TreasuryState {
    pub const SPACE: usize = 8 + 32 + 1 + 2 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 4 + 8 + 64;
}

#[account]
pub struct AssetRecord {
    pub treasury: Pubkey,
    pub index: u32,
    pub symbol: [u8; 16],
    pub quantity: u64,
    pub cost_basis: u64,
    pub timestamp: i64,
    pub source: [u8; 16],
    pub confirmed_by: Pubkey,
    pub bump: u8,
}

impl AssetRecord {
    pub const SPACE: usize = 8 + 32 + 4 + 16 + 8 + 8 + 8 + 16 + 32 + 1;
}

#[account]
pub struct DistributionSnapshot {
    pub treasury: Pubkey,
    pub epoch: u64,
    pub total_supply: u64,
    pub total_claimable_sol: u64,
    pub total_claimable_usdc: u64,
    pub created_at: i64,
    pub claimed_count: u32,
    pub bump: u8,
}

impl DistributionSnapshot {
    pub const SPACE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 4 + 1;
}

#[account]
pub struct HolderClaim {
    pub treasury: Pubkey,
    pub holder: Pubkey,
    pub epoch: u64,
    pub amount_sol: u64,
    pub amount_usdc: u64,
    pub claimed: bool,
    pub bump: u8,
}

impl HolderClaim {
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
}
