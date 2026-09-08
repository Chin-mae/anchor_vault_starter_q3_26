use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Vault does not contain enough withdrawable lamports")]
    InsufficientFunds,
}
