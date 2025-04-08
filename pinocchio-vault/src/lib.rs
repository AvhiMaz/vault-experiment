use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, find_program_address},
    ProgramResult,
};

const SYSTEM_PROGRAM_ID: Pubkey = [0u8; 32]; 

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let signer = &accounts[0];
    let vault_account = &accounts[1];

    if !signer.is_signer() {
        msg!("Error: Missing required signature.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (vault_pda, _vault_bump) = find_program_address(
        &[b"vault", signer.key().as_ref()],
        program_id,
    );

    if vault_account.key() != &vault_pda {
        msg!("provided vault account does not match derived PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    if unsafe { *vault_account.owner() } != SYSTEM_PROGRAM_ID {
        msg!("vault is not a system-owned account");
        return Err(ProgramError::IllegalOwner);
    }

    msg!("vault PDA is valid and system-owned");
    Ok(())
}

