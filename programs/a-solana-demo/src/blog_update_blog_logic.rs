use anchor_lang::prelude::*;

use crate::event::BlogUpdated;
use crate::state::Blog;
use crate::context::UpdateBlog;

pub(crate) fn verify(
    name: String,
    blog: &Account<Blog>,
    ctx: &Context<UpdateBlog>,
) -> BlogUpdated {
    let _ = ctx;
    BlogUpdated {
        owner: blog.owner.clone(),
        version: blog.version,
        name,
    }
}

pub(crate) fn mutate(
    blog_updated: &BlogUpdated,
    blog: &mut Account<Blog>,
) {
    blog.name = blog_updated.name.clone();
}
