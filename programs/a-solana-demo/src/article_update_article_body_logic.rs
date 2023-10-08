use anchor_lang::prelude::*;

use crate::event::ArticleBodyUpdated;
use crate::state::Article;
use crate::context::UpdateArticleBody;

pub(crate) fn verify(
    body: String,
    article: &Account<Article>,
    ctx: &Context<UpdateArticleBody>,
) -> ArticleBodyUpdated {
    let _ = ctx;
    ArticleBodyUpdated {
        article_id: article.article_id.clone(),
        version: article.version,
        body,
    }
}

pub(crate) fn mutate(
    article_body_updated: &ArticleBodyUpdated,
    article: &mut Account<Article>,
) {
    article.body = article_body_updated.body.clone();
}
