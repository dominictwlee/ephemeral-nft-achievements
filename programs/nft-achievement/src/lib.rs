mod state;
mod util;

use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token,
    token::{
        initialize_account as initialize_token_account, initialize_mint, mint_to, set_authority,
        InitializeAccount, InitializeMint, Mint, MintTo, SetAuthority, Token,
    },
};
use spl_token::instruction::AuthorityType;

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
            &ctx.accounts.issuer_authority,
            anchor_spl::token::Mint::LEN,
            None,
        )?;

        initialize_mint(cpi_context_initialize_mint, 0, &achievement.key(), None)?;

        achievement.issuer = ctx.accounts.issuer.key();
        achievement.owner = ctx.accounts.issuer.key();
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
        achievement_bump: u8,
        token_holding_bump: u8,
    ) -> ProgramResult {
        let seeds = &[
            b"achievement" as &[u8],
            &ctx.accounts.mint.key().to_bytes(),
            &[achievement_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let (cpi_context_initialize_token, cpi_context_mint_to, cpi_context_set_authority) =
            ctx.accounts.to_signed_cpi_contexts(signer_seeds);

        util::create_account(
            &anchor_spl::token::ID,
            &ctx.accounts.token_holding,
            &ctx.accounts.sysvar_rent,
            &ctx.accounts.system_program,
            &ctx.accounts.issuer_authority,
            anchor_spl::token::TokenAccount::LEN,
            Some(&[
                b"achievement_token_holding",
                ctx.accounts.mint.key().as_ref(),
                ctx.accounts.recipient.key().as_ref(),
                &[token_holding_bump],
            ]),
        )?;

        initialize_token_account(cpi_context_initialize_token)?;
        mint_to(cpi_context_mint_to, 1)?;
        set_authority(cpi_context_set_authority, AuthorityType::MintTokens, None)?;

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
        payer = issuer_authority,
        bump = args.bump,
        space = Achievement::max_space()
    )]
    pub achievement: Account<'info, Achievement>,

    #[account(mut, signer)]
    pub mint: AccountInfo<'info>,

    pub issuer: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
    pub issuer_authority: Signer<'info>,
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
#[instruction(achievement_bump: u8, token_holding_bump: u8)]
pub struct GrantAchievement<'info> {
    #[account(has_one = mint)]
    pub achievement: Account<'info, Achievement>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_holding: AccountInfo<'info>,

    pub issuer: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
    pub issuer_authority: Signer<'info>,
    pub sysvar_rent: Sysvar<'info, Rent>,
    pub sysvar_clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'a, 'b, 'c, 'info> GrantAchievement<'info> {
    fn to_signed_cpi_contexts(
        &self,
        signer_seeds: &'a [&'b [&'c [u8]]; 1],
    ) -> (
        CpiContext<'a, 'b, 'c, 'info, InitializeAccount<'info>>,
        CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>,
        CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>,
    ) {
        let cpi_initialize_token_accounts = InitializeAccount {
            account: self.token_holding.to_account_info().clone(),
            authority: self.achievement.to_account_info().clone(),
            mint: self.mint.to_account_info().clone(),
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

        let cpi_initialize_token = CpiContext::new_with_signer(
            self.token_program.to_account_info().clone(),
            cpi_initialize_token_accounts,
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

        (cpi_initialize_token, cpi_mint_to, cpi_set_authority)
    }
}
