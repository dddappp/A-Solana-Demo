use anchor_lang::prelude::*;

use crate::event::TagCreated;
use crate::state::Tag;
use crate::context::CreateTag;

pub(crate) fn verify(
    name: String,
    description: String,
    ctx: &Context<CreateTag>,
) -> TagCreated {
    let _ = ctx;
    TagCreated {
        name,
        description,
    }
}

pub(crate) fn mutate(
    tag_created: &TagCreated,
) -> Tag {
    Tag {
        name: tag_created.name.clone(),
        version: 0,
        description: tag_created.description.clone(),
    }
}
