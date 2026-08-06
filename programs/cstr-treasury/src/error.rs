use anchor_lang::prelude::*;

#[error_code]
pub enum TreasuryError {
    #[msg("The signer is not authorized to perform this action")]
    Unauthorized,
    #[msg("The amount must be greater than zero")]
    InvalidAmount,
    #[msg("The requested distribution epoch has already been claimed")]
    AlreadyClaimed,
    #[msg("The requested snapshot does not exist")]
    SnapshotNotFound,
    #[msg("The treasury is paused")]
    Paused,
}
