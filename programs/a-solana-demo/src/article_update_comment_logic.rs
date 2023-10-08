use anchor_lang::prelude::*;

use crate::event::CommentUpdated;
use crate::state::Article;
use crate::state::Comment;
use crate::context::UpdateComment;

pub(crate) fn verify(
    commenter: String,
    body: String,
    owner: Pubkey,
    article: &Account<Article>,
    comment: &Account<Comment>,
    ctx: &Context<UpdateComment>,
) -> CommentUpdated {
    let _ = ctx;
    CommentUpdated {
        article_id: article.article_id.clone(),
        version: article.version,
        comment: comment.key(),
        commenter,
        body,
        owner,
    }
}

pub(crate) fn mutate(
    comment_updated: &CommentUpdated,
    article: &mut Account<Article>,
    comment: &mut Account<Comment>,
) {
    comment.commenter = comment_updated.commenter.clone();
    comment.body = comment_updated.body.clone();
    comment.owner = comment_updated.owner.clone();
    // Here you can add code to operate on the aggregate root...
    let _ = article;
}
