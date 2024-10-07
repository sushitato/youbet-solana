use anchor_lang::prelude::*;

#[error_code]
pub enum YouBetError {
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Value less zero")]
    ValueLessZero,
}
