use anchor_lang::constant;
use anchor_lang::prelude::*;

#[constant]
pub const REWARD_PREFIX: &str = "REWARD";

#[account]
pub struct RewardAccount {
    pub reward_amount: u64,
    pub accumulated_amount: u64,
}

impl RewardAccount {
    pub fn space() -> usize {
        8 // default
        + 8
        + 8
    }
}
