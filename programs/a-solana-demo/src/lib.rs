use anchor_lang::prelude::*;

declare_id!("9TFvj5xg3X7URQGE8PYVEtb2HZ9mrGXRiWqmsdRX6xS3");

#[program]
pub mod a_solana_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
