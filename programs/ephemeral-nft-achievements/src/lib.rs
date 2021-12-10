mod state;
mod util;

use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{InitializeMint, Token};

declare_id!("AHQXfwGoMKFrhom1ZbkMStrFQkkZhqr31Xe7MF4bjtka");

#[program]
pub mod ephemeral_nft_achievements {

    use anchor_spl::token::initialize_mint;

    use super::*;
    pub fn create_achievement(
        ctx: Context<CreateAchievement>,
        args: CreateAchievementArgs,
    ) -> ProgramResult {
        let cpi_context_initialize_mint = ctx.accounts.to_cpi_contexts();
        let achievement = &mut ctx.accounts.achievement;
        let current_timestamp = ctx.accounts.sysvar_clock.unix_timestamp;

        util::create_account(
            &anchor_spl::token::ID,
            &ctx.accounts.mint,
            &ctx.accounts.sysvar_rent,
            &ctx.accounts.system_program,
            &ctx.accounts.granter_authority,
            anchor_spl::token::Mint::LEN,
            None,
        )?;

        initialize_mint(cpi_context_initialize_mint, 0, &achievement.key(), None)?;

        achievement.granter = ctx.accounts.granter.key();
        achievement.recipient = ctx.accounts.recipient.key();
        achievement.current_owner = ctx.accounts.recipient.key();
        achievement.mint = ctx.accounts.mint.key();
        achievement.tier = args.tier;
        achievement.bump = args.bump;
        achievement.created_at = current_timestamp;
        achievement.expires_at = current_timestamp + args.validity_length;
        achievement.uri = args.uri;
        achievement.max_transfer_count = args.max_transfer_count.unwrap_or(1);

        emit!(AchievementCreated {
            pubkey: achievement.key(),
        });

        Ok(())
    }
}

#[event]
pub struct AchievementCreated {
    pubkey: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: CreateAchievementArgs)]
pub struct CreateAchievement<'info> {
    #[account(
        init,
        seeds = [
            b"achievement",
            mint.key().to_bytes().as_ref(),

        ],
        payer = granter_authority,
        bump = args.bump,
        space = Achievement::max_space()
    )]
    pub achievement: Account<'info, Achievement>,

    #[account(mut, signer)]
    pub mint: AccountInfo<'info>,

    pub granter: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
    pub granter_authority: Signer<'info>,
    pub sysvar_rent: Sysvar<'info, Rent>,
    pub sysvar_clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct CreateAchievementArgs {
    pub tier: Tier,
    pub validity_length: i64,
    pub uri: String,
    pub bump: u8,
    pub max_transfer_count: Option<u8>,
}

impl<'a, 'b, 'c, 'info> CreateAchievement<'info> {
    fn to_cpi_contexts(&self) -> CpiContext<'a, 'b, 'c, 'info, InitializeMint<'info>> {
        let cpi_accounts_initialize_mint = InitializeMint {
            mint: self.mint.to_account_info().clone(),
            rent: self.sysvar_rent.to_account_info().clone(),
        };

        CpiContext::new(
            self.token_program.to_account_info().clone(),
            cpi_accounts_initialize_mint,
        )
    }
}
