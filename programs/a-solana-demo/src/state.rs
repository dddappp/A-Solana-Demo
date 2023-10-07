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

#[account]
#[derive(InitSpace)]
pub struct Article {
    pub article_id: u128,
    pub version: u64,
    #[max_len(200)]
    pub title: String,
    #[max_len(2000)]
    pub body: String,
    pub owner: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub article_id: u128,
    pub comment_seq_id: u64,
    #[max_len(100)]
    pub commenter: String,
    #[max_len(500)]
    pub body: String,
    pub owner: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Blog {
    pub owner: Pubkey,
    pub version: u64,
    #[max_len(200)]
    pub name: String,
}

