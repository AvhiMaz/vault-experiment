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

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }

}
