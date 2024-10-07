use anchor_lang::prelude::*;

pub const WALLET_PREFIX: &str = "WALLET";

#[account]
pub struct WalletAccount {
    pub github: String,
    pub user_point: u32,
    pub completed_tasks: Vec<String>,
}

impl WalletAccount {
    pub fn space() -> usize {
        8 // default
        + 64 // githubid
        + 4 // user_point
        + 64 * 10 // completed_taskss
    }
}
