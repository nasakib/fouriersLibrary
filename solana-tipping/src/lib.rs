use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};

/// Tipping Instruction payload
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TipInstruction {
    /// Amount of lamports to tip
    pub amount: u64,
    /// Minimum harmony score required (0-100 mapped to 0-255 for simplicity)
    pub min_harmony_threshold: u8, 
}

// Entry point of the Solana program
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Spotlight Local: Connectome Micropayment Tipping Program");

    // Decode instruction data
    let instruction = TipInstruction::try_from_slice(instruction_data)?;

    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let tipper_info = next_account_info(account_info_iter)?;
    let creator_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Basic verification
    if !tipper_info.is_signer {
        msg!("Error: Tipper must sign the transaction");
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    // In a full implementation, we would verify the creator's harmony score on-chain via an Oracle
    // For this MVP, we log the minimum threshold requested.
    msg!("Tipping {} lamports (Threshold requirement: {}/255 harmony)", instruction.amount, instruction.min_harmony_threshold);

    // Transfer lamports from tipper to creator
    invoke(
        &system_instruction::transfer(
            tipper_info.key,
            creator_info.key,
            instruction.amount,
        ),
        &[
            tipper_info.clone(),
            creator_info.clone(),
            system_program_info.clone(),
        ],
    )?;

    msg!("Tip transferred successfully! Shared Wealth Generation empowered.");

    Ok(())
}
