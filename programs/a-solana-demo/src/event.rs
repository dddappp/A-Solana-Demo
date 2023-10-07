use anchor_lang::prelude::*;

#[event]
pub struct TagCreated {
    pub name: String,
    pub description: String,
}

#[event]
pub struct TagUpdated {
    pub name: String,
    pub version: u64,
    pub description: String,
}

#[event]
pub struct CommentAdded {
    pub article_id: u128,
    pub version: u64,
    pub comment_seq_id: u64,
    pub commenter: String,
    pub body: String,
    pub owner: Pubkey,
}

#[event]
pub struct ArticleCreated {
    pub article_id: u128,
    pub title: String,
    pub body: String,
    pub owner: Pubkey,
}

#[event]
pub struct ArticleUpdated {
    pub article_id: u128,
    pub version: u64,
    pub title: String,
    pub body: String,
    pub owner: Pubkey,
}

