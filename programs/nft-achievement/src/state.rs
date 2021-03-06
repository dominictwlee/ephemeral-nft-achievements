use account_util::MAX_URI_LENGTH;
use anchor_lang::prelude::*;

pub trait MaxSpace {
    fn max_space() -> usize;
}

#[account]
#[derive(Default)]
pub struct Achievement {
    pub issuer: Pubkey,
    pub recipient: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub tier: Tier,
    pub bump: u8,
    pub created_at: i64,
    pub expires_at: i64,
    pub transferable: bool,
    pub transfer_count: u8,
    pub max_transfer_count: u8,
    pub uri: String,
}

impl MaxSpace for Achievement {
    fn max_space() -> usize {
        (32 * 4) + 4 + 1 + 8 + 8 + 3 + MAX_URI_LENGTH
    }
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub enum Tier {
    Trivial = 1,
    Minor = 2,
    Major = 3,
    Critical = 5,
    Innovative = 8,
}

impl Default for Tier {
    fn default() -> Self {
        Self::Trivial
    }
}
