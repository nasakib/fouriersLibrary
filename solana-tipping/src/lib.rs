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
pub enum TippingInstruction {
    /// Single tip
    Tip {
        /// Amount of lamports to tip
        amount: u64,
        /// Minimum harmony score required (0-100 mapped to 0-255 for simplicity)
        min_harmony_threshold: u8, 
    },
    /// Community Drop (Batch Tip)
    BatchTip {
        /// Amount of lamports to tip per creator
        amount_per_creator: u64,
        /// Number of creators to tip (passed via accounts)
        creator_count: u8,
    }
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
    let instruction = TippingInstruction::try_from_slice(instruction_data)?;

    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let tipper_info = next_account_info(account_info_iter)?;

    // Basic verification
    if !tipper_info.is_signer {
        msg!("Error: Tipper must sign the transaction");
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    let system_program_info = accounts.last().ok_or(solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;

    match instruction {
        TippingInstruction::Tip { amount, min_harmony_threshold } => {
            let creator_info = next_account_info(account_info_iter)?;
            
            msg!("Tipping {} lamports (Threshold requirement: {}/255 harmony)", amount, min_harmony_threshold);

            invoke(
                &system_instruction::transfer(
                    tipper_info.key,
                    creator_info.key,
                    amount,
                ),
                &[
                    tipper_info.clone(),
                    creator_info.clone(),
                    system_program_info.clone(),
                ],
            )?;

            msg!("Tip transferred successfully! Shared Wealth Generation empowered.");
        },
        TippingInstruction::BatchTip { amount_per_creator, creator_count } => {
            msg!("Initiating Community Drop: {} lamports to {} creators", amount_per_creator, creator_count);
            
            for _ in 0..creator_count {
                let creator_info = next_account_info(account_info_iter)?;
                invoke(
                    &system_instruction::transfer(
                        tipper_info.key,
                        creator_info.key,
                        amount_per_creator,
                    ),
                    &[
                        tipper_info.clone(),
                        creator_info.clone(),
                        system_program_info.clone(),
                    ],
                )?;
            }
            msg!("Community Drop completed successfully!");
        }
    }

    Ok(())
}
