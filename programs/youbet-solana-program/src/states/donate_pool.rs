use anchor_lang::prelude::*;

pub const DONATE_POOL_PREFIX: &str = "DONATE_POOL";

#[account]
pub struct DonatePoolAccount {}

impl DonatePoolAccount {
    pub fn space() -> usize {
        8 // default
    }
}
