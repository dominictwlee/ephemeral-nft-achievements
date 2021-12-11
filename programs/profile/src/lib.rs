mod state;

use account_util::MaxSpace;
use anchor_lang::prelude::*;
use state::*;

declare_id!("Gs9xfexZHrVAfSNZLAfaeu8VW8vnhxEzn88PYQQxbGXT");

#[program]
pub mod profile {
    use super::*;
    pub fn create(ctx: Context<Create>, args: CreateArgs) -> ProgramResult {
        let profile = &mut ctx.accounts.profile;

        profile.alias = args.alias;
        profile.bump = args.bump;
        profile.details_uri = args.details_uri.unwrap_or_default();
        profile.owner = ctx.accounts.owner.key();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(args: CreateArgs)]
pub struct Create<'info> {
    #[account(
        init,
        seeds = [
            b"profile",
            args.alias.as_bytes()
        ],
        payer = owner,
        bump = args.bump,
        space = Profile::max_space()
    )]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct CreateArgs {
    pub alias: String,
    pub bump: u8,
    pub details_uri: Option<String>,
}
