use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;

mod tag_create_tag_logic;
mod tag_update_tag_logic;
mod article_create_article_logic;
mod article_update_article_logic;

declare_id!("9TFvj5xg3X7URQGE8PYVEtb2HZ9mrGXRiWqmsdRX6xS3");

#[program]
pub mod a_solana_demo {
    use super::*;

    pub fn create_tag(
        ctx: Context<CreateTag>,
        name: String,
        description: String,
    ) -> Result<()> {
        let tag_created = tag_create_tag_logic::verify(
            name.clone(),
            description,
            &ctx,
        );
        assert_eq!(name, tag_created.name, "Name of event does not match");
        let mut tag = tag_create_tag_logic::mutate(
            &tag_created,
        );
        assert_eq!(name, tag.name, "Name of state does not match");
        tag.version = 0;
        *ctx.accounts.tag = tag;
        emit!(tag_created);

        Ok(())
    }

    pub fn update_tag(
        ctx: Context<UpdateTag>,
        description: String,
    ) -> Result<()> {
        let tag = &ctx.accounts.tag;
        let name = tag.name.clone();
        let old_version = tag.version;
        let tag_updated = tag_update_tag_logic::verify(
            description,
            tag,
            &ctx,
        );
        assert_eq!(name, tag_updated.name, "Name of event does not match");
        assert_eq!(old_version, tag_updated.version, "Version of event does not match");
        let tag = &mut ctx.accounts.tag;
        tag_update_tag_logic::mutate(
            &tag_updated,
            tag,
        );
        assert_eq!(name, tag.name, "Name of state does not match");
        tag.version = old_version + 1;
        emit!(tag_updated);

        Ok(())
    }

    pub fn create_article(
        ctx: Context<CreateArticle>,
        article_id: u128,
        title: String,
        body: String,
        owner: Pubkey,
    ) -> Result<()> {
        let article_created = article_create_article_logic::verify(
            article_id.clone(),
            title,
            body,
            owner,
            &ctx,
        );
        assert_eq!(article_id, article_created.article_id, "ArticleId of event does not match");
        let mut article = article_create_article_logic::mutate(
            &article_created,
        );
        assert_eq!(article_id, article.article_id, "ArticleId of state does not match");
        article.version = 0;
        *ctx.accounts.article = article;
        emit!(article_created);

        Ok(())
    }

    pub fn update_article(
        ctx: Context<UpdateArticle>,
        title: String,
        body: String,
        owner: Pubkey,
    ) -> Result<()> {
        let article = &ctx.accounts.article;
        let article_id = article.article_id.clone();
        let old_version = article.version;
        let article_updated = article_update_article_logic::verify(
            title,
            body,
            owner,
            article,
            &ctx,
        );
        assert_eq!(article_id, article_updated.article_id, "ArticleId of event does not match");
        assert_eq!(old_version, article_updated.version, "Version of event does not match");
        let article = &mut ctx.accounts.article;
        article_update_article_logic::mutate(
            &article_updated,
            article,
        );
        assert_eq!(article_id, article.article_id, "ArticleId of state does not match");
        article.version = old_version + 1;
        emit!(article_updated);

        Ok(())
    }

}
