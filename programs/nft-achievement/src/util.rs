use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    },
};
use std::convert::TryInto;

pub fn create_account<'a>(
    owner: &Pubkey,
    new_account_info: &AccountInfo<'a>,
    sysvar_rent: &Sysvar<'a, Rent>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    size: usize,
    signer_seeds: Option<&[&[u8]]>,
) -> Result<(), ProgramError> {
    let required_lamports = sysvar_rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!(
            "Transfer {} lamports to the new account {}",
            required_lamports,
            new_account_info.key
        );
        invoke(
            &system_instruction::transfer(payer.key, new_account_info.key, required_lamports),
            &[
                payer.to_account_info(),
                new_account_info.clone(),
                system_program.to_account_info(),
            ],
        )?;
    }

    msg!("Allocate space for the account");
    if let Some(s) = signer_seeds {
        msg!("Allocate space for the account");
        invoke_signed(
            &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
            &[new_account_info.clone(), system_program.to_account_info()],
            &[s],
        )?;

        msg!("Assign the account to the owning program");
        invoke_signed(
            &system_instruction::assign(new_account_info.key, owner),
            &[new_account_info.clone(), system_program.to_account_info()],
            &[s],
        )?;
    } else {
        msg!("Allocate space for the account");
        invoke(
            &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
            &[new_account_info.clone(), system_program.to_account_info()],
        )?;

        msg!("Assign the account to the owning program");
        invoke(
            &system_instruction::assign(new_account_info.key, owner),
            &[new_account_info.clone(), system_program.to_account_info()],
        )?;
    }

    msg!("Completed assignation!");

    Ok(())
}
