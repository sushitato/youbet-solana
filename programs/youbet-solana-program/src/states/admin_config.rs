use anchor_lang::prelude::*;

pub const ADMIN_CONFIG_PREFIX: &str = "ADMIN_CONFIG";

#[account]
pub struct AdminConfigAccount {
    pub last_block_timestamp: i64,
    pub authority: Pubkey,
}

impl AdminConfigAccount {
    pub fn space() -> usize {
        8 // default
        +8
        + 32 // admin
    }
}
