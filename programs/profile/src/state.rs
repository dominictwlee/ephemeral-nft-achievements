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
