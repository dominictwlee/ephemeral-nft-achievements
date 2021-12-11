use account_util::{MaxSpace, MAX_PROFILE_NAME_LENGTH, MAX_URI_LENGTH};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub name: String,
    pub delegates: Vec<Pubkey>,
    pub details_uri: String,
}

impl MaxSpace for Profile {
    fn max_space() -> usize {
        MAX_PROFILE_NAME_LENGTH + 32 + 1 + 184 + MAX_URI_LENGTH
    }
}

#[account]
#[derive(Default)]
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
