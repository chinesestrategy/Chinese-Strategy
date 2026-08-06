use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("22222222222222222222222222222222");

#[program]
pub mod cstr_token {
    use super::*;

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
        transfer_fee_bps: u16,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = decimals;
        mint.mint_authority = ctx.accounts.authority.key();
        mint.freeze_authority = None;
        mint.supply = 0;
        mint.transfer_fee_bps = transfer_fee_bps;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority.key())]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum TokenError {
    #[msg("Transfer fee basis points must be between 0 and 10000")]
    InvalidFeeBps,
}
