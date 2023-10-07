use anchor_lang::prelude::*;

use crate::event::CommentAdded;
use crate::state::Article;
use crate::context::AddComment;

pub(crate) fn verify(
    comment_seq_id: u64,
    commenter: String,
    body: String,
    owner: Pubkey,
    article: &Account<Article>,
    ctx: &Context<AddComment>,
) -> CommentAdded {
    let _ = ctx;
    CommentAdded {
        article_id: article.article_id.clone(),
        version: article.version,
        comment_seq_id,
        commenter,
        body,
        owner,
    }
}

pub(crate) fn mutate(
    comment_added: &CommentAdded,
    article: &mut Account<Article>,
) {
        //...
}
