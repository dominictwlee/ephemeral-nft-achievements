use anchor_lang::prelude::*;

pub const MAX_URI_LENGTH: usize = 200;

pub trait MaxSpace {
    fn max_space() -> usize;
}

#[account]
#[derive(Default)]
pub struct Achievement {
    pub granter: Pubkey,
    pub recipient: Pubkey,
    pub current_owner: Pubkey,
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
        32 + 32 + 32 + 4 + 1 + 8 + 1 + 1 + MAX_URI_LENGTH
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
