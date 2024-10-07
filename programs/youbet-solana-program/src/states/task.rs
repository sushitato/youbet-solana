use anchor_lang::prelude::*;

pub const TASK_PREFIX: &str = "TASK";

#[account]
pub struct TaskAccount {
    pub task_id: String,
    pub name: String,
    pub completed: bool,
    pub project_id: String,
    pub project_bump: u8,
    pub task_completer: Pubkey,
}

impl TaskAccount {
    pub fn space() -> usize {
        8 // default
        + 64 // task_id
        + 64 // name
        + 1 // bool
        + 64 // project_id
        + 1 // u8
        + 32 // task_completer
    }
}
