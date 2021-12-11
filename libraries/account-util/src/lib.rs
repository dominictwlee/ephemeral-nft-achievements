use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    },
};
use std::{convert::TryInto, time::Duration};

pub const ONE_DAY_IN_SECONDS: Duration = Duration::from_secs(86400);
pub const MAX_ALIAS_LENGTH: usize = 30;
pub const MAX_URI_LENGTH: usize = 200;

pub trait MaxSpace {
    fn max_space() -> usize;
}

pub fn create_account<'a>(
    owner: &Pubkey,
    new_account_info: &AccountInfo<'a>,
    sysvar_rent: &Sysvar<'a, Rent>,
    system_program: &Program<'a, System>,
    payer: &Signer<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
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
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program.to_account_info()],
        &[signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, owner),
        &[new_account_info.clone(), system_program.to_account_info()],
        &[signer_seeds],
    )?;
    msg!("Completed assignation!");

    Ok(())
}
