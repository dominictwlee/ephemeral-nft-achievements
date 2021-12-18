mod state;
mod util;

use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token,
    token::{initialize_mint, InitializeMint, Mint, MintTo, SetAuthority, Token},
};

declare_id!("Ca2F3pQ66qNqgBfHBKD8oSXDS8hhQmNraEkHqr8kTB6H");

#[program]
pub mod nft_achievement {
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

        achievement.creator = ctx.accounts.creator.key();
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

    pub fn grant_achievement(
        ctx: Context<GrantAchievement>,
        args: GrantAchievementArgs,
    ) -> ProgramResult {
        let seeds = &[
            b"achievement" as &[u8],
            &ctx.accounts.mint.key().to_bytes(),
            &[args.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let (
            cpi_context_create_associated_token,
            cpi_context_initialize_mint,
            cpi_context_set_authority,
        ) = ctx.accounts.to_signed_cpi_contexts(signer_seeds);

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
            mint.key().as_ref(),
        ],
        payer = granter_authority,
        bump = args.bump,
        space = Achievement::max_space()
    )]
    pub achievement: Account<'info, Achievement>,

    #[account(mut, signer)]
    pub mint: AccountInfo<'info>,

    pub creator: AccountInfo<'info>,
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

#[derive(Accounts)]
#[instruction(args: GrantAchievementArgs)]
pub struct GrantAchievement<'info> {
    #[account(has_one = mint)]
    pub achievement: Account<'info, Achievement>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_holding: AccountInfo<'info>,

    pub granter: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
    pub granter_authority: Signer<'info>,
    pub sysvar_rent: Sysvar<'info, Rent>,
    pub sysvar_clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct GrantAchievementArgs {
    pub tier: Tier,
    pub validity_length: i64,
    pub uri: String,
    pub bump: u8,
    pub max_transfer_count: Option<u8>,
}

impl<'a, 'b, 'c, 'info> GrantAchievement<'info> {
    fn to_signed_cpi_contexts(
        &self,
        signer_seeds: &'a [&'b [&'c [u8]]; 1],
    ) -> (
        CpiContext<'a, 'b, 'c, 'info, associated_token::Create<'info>>,
        CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>,
        CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>,
    ) {
        let cpi_create_associated_token_accounts = associated_token::Create {
            payer: self.granter_authority.to_account_info().clone(),
            associated_token: self.token_holding.to_account_info().clone(),
            authority: self.achievement.to_account_info().clone(),
            mint: self.mint.to_account_info().clone(),
            system_program: self.system_program.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
            rent: self.sysvar_rent.to_account_info().clone(),
        };
        let cpi_mint_to_accounts = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.token_holding.to_account_info().clone(),
            authority: self.achievement.to_account_info().clone(),
        };
        let cpi_set_authority_accounts = SetAuthority {
            current_authority: self.achievement.to_account_info().clone(),
            account_or_mint: self.mint.to_account_info().clone(),
        };

        let cpi_create_associated_token = CpiContext::new_with_signer(
            self.token_program.to_account_info().clone(),
            cpi_create_associated_token_accounts,
            signer_seeds,
        );

        let cpi_mint_to = CpiContext::new_with_signer(
            self.token_program.to_account_info().clone(),
            cpi_mint_to_accounts,
            signer_seeds,
        );

        let cpi_set_authority = CpiContext::new_with_signer(
            self.token_program.to_account_info().clone(),
            cpi_set_authority_accounts,
            signer_seeds,
        );

        (cpi_create_associated_token, cpi_mint_to, cpi_set_authority)
    }
}
