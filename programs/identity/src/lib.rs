mod error;
mod state;

use account_util::{MaxSpace, MAX_PROFILE_NAME_LENGTH, MAX_URI_LENGTH};
use anchor_lang::prelude::*;
use error::IdentityError;
use state::*;

declare_id!("3U2mqSTuSjj9QkDRT2Wy8o7DAo8iWU2iyJzHfW3W7r5f");

#[program]
pub mod identity {
    use super::*;

    #[access_control(CreateProfile::validate(&args))]
    pub fn create_profile(ctx: Context<CreateProfile>, args: CreateProfileArgs) -> ProgramResult {
        let profile = &mut ctx.accounts.profile;

        profile.name = args.name;
        profile.bump = args.bump;
        profile.details_uri = args.details_uri.unwrap_or_default();
        profile.owner = ctx.accounts.owner.key();

        Ok(())
    }

    pub fn add_delegate(ctx: Context<AddDelegate>) -> ProgramResult {
        let profile = &mut ctx.accounts.profile;

        profile.delegates.push(ctx.accounts.delegate.key());

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(args: CreateProfileArgs)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        seeds = [
            b"profile",
            args.name.as_bytes()
        ],
        payer = owner,
        bump = args.bump,
        space = Profile::max_space()
    )]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProfile<'info> {
    pub fn validate(args: &CreateProfileArgs) -> ProgramResult {
        if args.name.chars().count() > MAX_PROFILE_NAME_LENGTH {
            return Err(IdentityError::ProfileNameCharLengthExceeded.into());
        }

        if let Some(uri) = &args.details_uri {
            if uri.chars().count() > MAX_URI_LENGTH {
                return Err(IdentityError::URICharLengthExceeded.into());
            }
        }

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct CreateProfileArgs {
    pub name: String,
    pub bump: u8,
    pub details_uri: Option<String>,
}

#[derive(Accounts)]
pub struct AddDelegate<'info> {
    #[account(mut, has_one = owner)]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
    #[account(signer)]
    pub delegate: AccountInfo<'info>,
}
