use anchor_lang::prelude::*;
use crate::states::Vault;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"state", signer.key().as_ref()],
        bump,
        space = 8 + Vault::INIT_SPACE
    )]
    pub vault_state: Account<'info, Vault>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]

    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl <'info> Init <'info> {
    pub fn init(&mut self, bumps: &InitBumps) -> Result<()> {
        
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
