use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateTag<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Tag::INIT_SPACE,
        seeds = [
            b"Tag",
            name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub tag: Account<'info, Tag>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTag<'info> {
    #[account(
        mut,
        seeds = [
            b"Tag",
            tag.name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub tag: Account<'info, Tag>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

