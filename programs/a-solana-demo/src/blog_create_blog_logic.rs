use anchor_lang::prelude::*;

use crate::event::BlogCreated;
use crate::state::Blog;
use crate::context::CreateBlog;

pub(crate) fn verify(
    owner: Pubkey,
    name: String,
    ctx: &Context<CreateBlog>,
) -> BlogCreated {
    let _ = ctx;
    BlogCreated {
        owner,
        name,
    }
}

pub(crate) fn mutate(
    blog_created: &BlogCreated,
) -> Blog {
    Blog {
        owner: blog_created.owner.clone(),
        version: 0,
        name: blog_created.name.clone(),
    }
}
