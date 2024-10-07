use anchor_lang::prelude::*;

pub const GITHUB_PREFIX: &str = "GITHUB";

#[account]
pub struct GithubAccount {
    pub wallet: Pubkey,
}

impl GithubAccount {
    pub fn space() -> usize {
        8 // default
        + 32 // wallet
    }
}
