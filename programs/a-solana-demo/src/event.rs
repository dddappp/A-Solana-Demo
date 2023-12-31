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
pub struct ArticleBodyUpdated {
    pub article_id: u128,
    pub version: u64,
    pub body: String,
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
pub struct CommentUpdated {
    pub article_id: u128,
    pub version: u64,
    pub comment: Pubkey,
    pub commenter: String,
    pub body: String,
    pub owner: Pubkey,
}

#[event]
pub struct BlogCreated {
    pub owner: Pubkey,
    pub name: String,
}

#[event]
pub struct BlogUpdated {
    pub owner: Pubkey,
    pub version: u64,
    pub name: String,
}

