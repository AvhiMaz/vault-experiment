#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
mod states;
mod instructions;

use instructions::*;

declare_id!("5hGhaKA4fbHGX2qkU6gPv8QnMSRE2jUJtNnaG5Dk65ZQ");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

}
