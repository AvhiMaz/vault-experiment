use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self},
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

use crate::states::{load_ix_data, DataLen};

#[repr(C)]
pub struct DeposiIxtData {
    pub amount: u64,
    pub bump: u8,
}

impl DataLen for DeposiIxtData {
    const LEN: usize = core::mem::size_of::<DeposiIxtData>();
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [deposit_acc, vault_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !deposit_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ix_data = load_ix_data::<DeposiIxtData>(data)?;

    let vault_pda = pubkey::create_program_address(
        &[
            "pinocchio_vault_pda".as_bytes(),
            deposit_acc.key(),
            &[ix_data.bump],
        ],
        &crate::ID,
    )?;
    if vault_acc.key() != &vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: deposit_acc,
        to: vault_acc,
        lamports: ix_data.amount * LAMPORTS_PER_SOL,
    }
    .invoke()?;

    Ok(())
}
