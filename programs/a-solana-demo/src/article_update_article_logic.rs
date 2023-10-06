use anchor_lang::prelude::*;

use crate::event::ArticleUpdated;
use crate::state::Article;
use crate::context::UpdateArticle;

pub(crate) fn verify(
    title: String,
    body: String,
    owner: Pubkey,
    article: &Account<Article>,
    ctx: &Context<UpdateArticle>,
) -> ArticleUpdated {
    let _ = ctx;
    ArticleUpdated {
        article_id: article.article_id.clone(),
        version: article.version,
        title,
        body,
        owner,
    }
}

pub(crate) fn mutate(
    article_updated: &ArticleUpdated,
    article: &mut Account<Article>,
) {
    article.title = article_updated.title.clone();
    article.body = article_updated.body.clone();
    article.owner = article_updated.owner.clone();
}
