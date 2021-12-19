use account_util::{MaxSpace, MAX_PROFILE_NAME_LENGTH, MAX_URI_LENGTH};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub achievements_created: u64,
    pub name: String,
    pub details_uri: String,
}

impl MaxSpace for Profile {
    fn max_space() -> usize {
        32 + 1 + 8 + MAX_PROFILE_NAME_LENGTH + MAX_URI_LENGTH
    }
}

#[account]
#[derive(Default)]
pub struct Claim {
    pub claim_type: ClaimType,
    pub profile: Pubkey,
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
        ClaimType::GitHub
    }
}
