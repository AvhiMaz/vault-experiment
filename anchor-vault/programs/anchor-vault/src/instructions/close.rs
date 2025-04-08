use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::states::Vault;

#[derive(Accounts)]

pub struct Close <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        close = signer,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]

    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl <'info> Close <'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program  = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let binding = self.vault_state.to_account_info().key();

        let seeds = &[
            b"vault",
            binding.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}
