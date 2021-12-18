use account_util::{MaxSpace, MAX_PROFILE_NAME_LENGTH, MAX_URI_LENGTH};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub github_verified: bool,
    pub email_verified: bool,
    pub linkedin_verified: bool,
    pub name: String,
    pub details_uri: String,
}

impl MaxSpace for Profile {
    fn max_space() -> usize {
        32 + 1 + 3 + MAX_PROFILE_NAME_LENGTH + 184 + MAX_URI_LENGTH
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Claim {
    pub claim_type: ClaimType,
    pub certifier: Pubkey,
    pub recipient: Pubkey,
    pub username: String,
}

#[account]
#[derive(Default)]
pub struct Certifier {
    pub authority: Pubkey,
    pub bump: u8,
    pub claim_type: ClaimType,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum ClaimType {
    GitHub,
    Email,
    Linkedin,
}

impl Default for ClaimType {
    fn default() -> Self {
        ClaimType::Email
    }
}
