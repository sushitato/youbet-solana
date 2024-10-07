use anchor_lang::constant;
use anchor_lang::prelude::*;

#[constant]
pub const PROJECT_PREFIX: &str = "PROJECT";
pub const PROJECT_USER_POINT_PREFIX: &str = "PROJECT_USER_POINT";

#[account]
pub struct ProjectAccount {
    pub project_id: String,
    pub name: String,
    pub total_project_points: u32,
    pub participaints: Vec<Pubkey>,
}

impl ProjectAccount {
    pub fn space() -> usize {
        8 // default
        + 64
        + 64
        + 32 * 10 // max 10 people
    }
}

#[account]
pub struct ProjectUserPointAccount {
    pub user_point: u32,
}

impl ProjectUserPointAccount {
    pub fn space() -> usize {
        8 // default
        + 4 // user_point
    }
}
