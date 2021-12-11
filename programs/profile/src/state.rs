use account_util::{MaxSpace, MAX_URI_LENGTH};
use anchor_lang::prelude::*;

const MAX_ALIAS_LENGTH: usize = 50;

#[account]
#[derive(Default)]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub alias: String,
    pub delegates: Vec<Pubkey>,
    pub details_uri: String,
}

impl MaxSpace for Profile {
    fn max_space() -> usize {
        MAX_ALIAS_LENGTH + 32 + 1 + 184 + MAX_URI_LENGTH
    }
}
