use anchor_lang::prelude::*;

use crate::event::TagUpdated;
use crate::state::Tag;
use crate::context::UpdateTag;

pub(crate) fn verify(
    description: String,
    tag: &Account<Tag>,
    ctx: &Context<UpdateTag>,
) -> TagUpdated {
    let _ = ctx;
    TagUpdated {
        name: tag.name.clone(),
        version: tag.version,
        description,
    }
}

pub(crate) fn mutate(
    tag_updated: &TagUpdated,
    tag: &mut Account<Tag>,
) {
    tag.description = tag_updated.description.clone();
}
