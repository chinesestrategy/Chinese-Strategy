use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod cstr_treasury {
    use super::*;

    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        authority: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = authority;
        treasury.bump = ctx.bumps.treasury;
        treasury.fee_bps = fee_bps;
        treasury.paused = false;
        treasury.total_sol_inflow = 0;
        treasury.total_usdc_inflow = 0;
        treasury.total_distributed_sol = 0;
        treasury.total_distributed_usdc = 0;
        treasury.current_epoch = 0;
        treasury.last_snapshot_slot = 0;
        treasury.asset_backing_count = 0;
        treasury.created_at = Clock::get()?.unix_timestamp;
        treasury.reserved = [0; 64];
        emit!(TreasuryInitialized {
            authority,
            fee_bps,
            timestamp: treasury.created_at,
        });
        Ok(())
    }

    pub fn deposit_fees(ctx: Context<DepositFees>, amount: u64) -> Result<()> {
        require!(amount > 0, error::TreasuryError::InvalidAmount);
        require!(!ctx.accounts.treasury.paused, error::TreasuryError::Paused);
        let treasury = &mut ctx.accounts.treasury;
        treasury.total_sol_inflow = treasury.total_sol_inflow.checked_add(amount).unwrap();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
        emit!(TreasuryMovement {
            kind: b"fee_deposit".to_vec(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }

    pub fn deposit_dead_pool_funds(ctx: Context<DepositDeadPoolFunds>, amount: u64) -> Result<()> {
        require!(amount > 0, error::TreasuryError::InvalidAmount);
        require!(!ctx.accounts.treasury.paused, error::TreasuryError::Paused);
        let treasury = &mut ctx.accounts.treasury;
        treasury.total_sol_inflow = treasury.total_sol_inflow.checked_add(amount).unwrap();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
        emit!(TreasuryMovement {
            kind: b"dead_pool_deposit".to_vec(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }

    pub fn record_asset_purchase(
        ctx: Context<RecordAssetPurchase>,
        symbol: String,
        quantity: u64,
        cost_basis: u64,
        source: String,
    ) -> Result<()> {
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.treasury.authority);
        let treasury = &mut ctx.accounts.treasury;
        let index = treasury.asset_backing_count;
        let asset = &mut ctx.accounts.asset_record;
        asset.treasury = treasury.key();
        asset.index = index;
        asset.symbol = fixed_bytes_from_string(&symbol, 16)?;
        asset.quantity = quantity;
        asset.cost_basis = cost_basis;
        asset.timestamp = Clock::get()?.unix_timestamp;
        asset.source = fixed_bytes_from_string(&source, 16)?;
        asset.confirmed_by = ctx.accounts.authority.key();
        asset.bump = ctx.bumps.asset_record;
        treasury.asset_backing_count = treasury.asset_backing_count.checked_add(1).unwrap();
        emit!(AssetPurchaseRecorded {
            symbol: symbol.clone(),
            quantity,
            cost_basis,
            timestamp: asset.timestamp,
        });
        Ok(())
    }

    pub fn distribute_to_holders(
        ctx: Context<DistributeToHolders>,
        total_supply: u64,
        total_claimable_sol: u64,
        total_claimable_usdc: u64,
    ) -> Result<()> {
        require_keys_eq!(ctx.accounts.authority.key(), ctx.accounts.treasury.authority);
        let treasury = &mut ctx.accounts.treasury;
        treasury.current_epoch = treasury.current_epoch.checked_add(1).unwrap();
        treasury.last_snapshot_slot = Clock::get()?.slot;
        let snapshot = &mut ctx.accounts.snapshot;
        snapshot.treasury = treasury.key();
        snapshot.epoch = treasury.current_epoch;
        snapshot.total_supply = total_supply;
        snapshot.total_claimable_sol = total_claimable_sol;
        snapshot.total_claimable_usdc = total_claimable_usdc;
        snapshot.created_at = Clock::get()?.unix_timestamp;
        snapshot.claimed_count = 0;
        snapshot.bump = ctx.bumps.snapshot;
        emit!(DistributionSnapshotCreated {
            epoch: snapshot.epoch,
            total_supply,
            total_claimable_sol,
            total_claimable_usdc,
        });
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, holder_balance: u64) -> Result<()> {
        let treasury = &ctx.accounts.treasury;
        let snapshot = &ctx.accounts.snapshot;
        require!(!ctx.accounts.claim_state.claimed, error::TreasuryError::AlreadyClaimed);
        let claimable = if snapshot.total_supply == 0 {
            0
        } else {
            snapshot.total_claimable_sol.checked_mul(holder_balance).unwrap().checked_div(snapshot.total_supply).unwrap()
        };
        let claim_state = &mut ctx.accounts.claim_state;
        claim_state.claimed = true;
        claim_state.amount_sol = claimable;
        claim_state.amount_usdc = 0;
        let transfer_amount = claimable;
        if transfer_amount > 0 {
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: treasury.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            };
            let seeds = &[b"treasury", treasury.key.as_ref(), &[treasury.bump]];
            let signer = [&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), cpi_accounts, &signer);
            anchor_lang::system_program::transfer(cpi_ctx, transfer_amount)?;
        }
        emit!(RewardsClaimed {
            epoch: snapshot.epoch,
            holder: ctx.accounts.holder.key(),
            amount: claimable,
        });
        Ok(())
    }
}

fn fixed_bytes_from_string(value: &str, size: usize) -> Result<[u8; 16]> {
    let mut bytes = [0u8; 16];
    let bytes_value = value.as_bytes();
    let len = bytes_value.len().min(size);
    bytes[..len].copy_from_slice(&bytes_value[..len]);
    Ok(bytes)
}

#[event]
pub struct TreasuryInitialized {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct TreasuryMovement {
    pub kind: Vec<u8>,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct AssetPurchaseRecorded {
    pub symbol: String,
    pub quantity: u64,
    pub cost_basis: u64,
    pub timestamp: i64,
}

#[event]
pub struct DistributionSnapshotCreated {
    pub epoch: u64,
    pub total_supply: u64,
    pub total_claimable_sol: u64,
    pub total_claimable_usdc: u64,
}

#[event]
pub struct RewardsClaimed {
    pub epoch: u64,
    pub holder: Pubkey,
    pub amount: u64,
}
