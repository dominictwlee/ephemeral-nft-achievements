mod state;
mod util;

use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("AHQXfwGoMKFrhom1ZbkMStrFQkkZhqr31Xe7MF4bjtka");

#[program]
pub mod ephemeral_nft_achievements {
    use super::*;
    pub fn create_achievement(
        ctx: Context<CreateAchievement>,
        args: CreateAchievementArgs,
    ) -> ProgramResult {
        let achievement = &mut ctx.accounts.achievement;
        let current_timestamp = ctx.accounts.sysvar_clock.unix_timestamp;

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
    )]
    pub achievement: Account<'info, Achievement>,

    pub mint: Account<'info, Mint>,

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
