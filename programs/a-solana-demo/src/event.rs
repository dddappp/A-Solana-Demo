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

