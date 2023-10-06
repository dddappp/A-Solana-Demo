use anchor_lang::prelude::*;

use crate::event::ArticleCreated;
use crate::state::Article;
use crate::context::CreateArticle;

pub(crate) fn verify(
    article_id: u128,
    title: String,
    body: String,
    owner: Pubkey,
    ctx: &Context<CreateArticle>,
) -> ArticleCreated {
    let _ = ctx;
    ArticleCreated {
        article_id,
        title,
        body,
        owner,
    }
}

pub(crate) fn mutate(
    article_created: &ArticleCreated,
) -> Article {
    Article {
        article_id: article_created.article_id.clone(),
        version: 0,
        title: article_created.title.clone(),
        body: article_created.body.clone(),
        owner: article_created.owner.clone(),
    }
}
