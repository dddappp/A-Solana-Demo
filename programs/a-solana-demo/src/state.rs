use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Tag {
    #[max_len(50)]
    pub name: String,
    pub version: u64,
    #[max_len(100)]
    pub description: String,
}
