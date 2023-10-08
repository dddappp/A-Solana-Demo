use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;

mod tag_create_tag_logic;
mod tag_update_tag_logic;
mod article_add_comment_logic;
mod article_update_comment_logic;
mod article_create_article_logic;
mod article_update_article_logic;
mod blog_create_blog_logic;
mod blog_update_blog_logic;

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

    pub fn add_comment(
        ctx: Context<AddComment>,
        comment_seq_id: u64,
        commenter: String,
        body: String,
        owner: Pubkey,
    ) -> Result<()> {
        let article = &ctx.accounts.article;
        let article_id = article.article_id.clone();
        let old_version = article.version;
        let comment_added = article_add_comment_logic::verify(
            comment_seq_id,
            commenter,
            body,
            owner,
            article,
            &ctx,
        );
        assert_eq!(article_id, comment_added.article_id, "ArticleId of event does not match");
        assert_eq!(old_version, comment_added.version, "Version of event does not match");
        let article = &mut ctx.accounts.article;
        let comment = &mut ctx.accounts.comment;
        article_add_comment_logic::mutate(
            &comment_added,
            article,
            comment,
        );
        assert_eq!(article_id, article.article_id, "ArticleId of state does not match");
        article.version = old_version + 1;
        comment.article_id = article_id.clone();
        comment.comment_seq_id = comment_seq_id.clone();
        emit!(comment_added);

        Ok(())
    }

    pub fn update_comment(
        ctx: Context<UpdateComment>,
        commenter: String,
        body: String,
        owner: Pubkey,
    ) -> Result<()> {
        let article = &ctx.accounts.article;
        let article_id = article.article_id.clone();
        let old_version = article.version;
        let comment = &ctx.accounts.comment;
        assert_eq!(article_id, comment.article_id, "ArticleId of entity does not match");
        let comment_seq_id = comment.comment_seq_id;
        let comment_updated = article_update_comment_logic::verify(
            commenter,
            body,
            owner,
            article,
            comment,
            &ctx,
        );
        assert_eq!(article_id, comment_updated.article_id, "ArticleId of event does not match");
        assert_eq!(old_version, comment_updated.version, "Version of event does not match");
        let article = &mut ctx.accounts.article;
        let comment = &mut ctx.accounts.comment;
        article_update_comment_logic::mutate(
            &comment_updated,
            article,
            comment,
        );
        assert_eq!(article_id, article.article_id, "ArticleId of state does not match");
        article.version = old_version + 1;
        assert_eq!(article_id, comment.article_id, "ArticleId of state does not match");
        assert_eq!(comment_seq_id, comment.comment_seq_id, "CommentSeqId of state does not match");
        emit!(comment_updated);

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

    pub fn create_blog(
        ctx: Context<CreateBlog>,
        name: String,
    ) -> Result<()> {
        let owner = *ctx.accounts.authority.key;
        let blog_created = blog_create_blog_logic::verify(
            owner.clone(),
            name,
            &ctx,
        );
        assert_eq!(owner, blog_created.owner, "Owner of event does not match");
        let mut blog = blog_create_blog_logic::mutate(
            &blog_created,
        );
        assert_eq!(owner, blog.owner, "Owner of state does not match");
        blog.version = 0;
        *ctx.accounts.blog = blog;
        emit!(blog_created);

        Ok(())
    }

    pub fn update_blog(
        ctx: Context<UpdateBlog>,
        name: String,
    ) -> Result<()> {
        let blog = &ctx.accounts.blog;
        let owner = blog.owner.clone();
        let old_version = blog.version;
        let blog_updated = blog_update_blog_logic::verify(
            name,
            blog,
            &ctx,
        );
        assert_eq!(owner, blog_updated.owner, "Owner of event does not match");
        assert_eq!(old_version, blog_updated.version, "Version of event does not match");
        let blog = &mut ctx.accounts.blog;
        blog_update_blog_logic::mutate(
            &blog_updated,
            blog,
        );
        assert_eq!(owner, blog.owner, "Owner of state does not match");
        blog.version = old_version + 1;
        emit!(blog_updated);

        Ok(())
    }

}
