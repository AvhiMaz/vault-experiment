use crate::instruction::{self, VaultInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    pubkey::Pubkey, ProgramError, ProgramResult,
};
use pinocchio_log::log;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator_variant, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData);

    match VaultInstruction::try_from(discriminator_variant)? {
        VaultInstruction::Deposit => {
            log!("Ix:0");
            instruction::process_deposit(accounts, instruction_data);
        }
        VaultInstruction::withdraw => {
            log!("Ix:1");
            instruction::process_withdraw(account, instruction_data);
        }
    }

    Ok(())
}
