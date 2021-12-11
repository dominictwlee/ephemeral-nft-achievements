use anchor_lang::prelude::*;

#[error]
pub enum ProfileError {
    #[msg("Alias character length exceeded")]
    AliasCharLengthExceeded,

    #[msg("URI character length exceeded")]
    URICharLengthExceeded,
}
