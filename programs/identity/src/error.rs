use anchor_lang::prelude::*;

#[error]
pub enum IdentityError {
    #[msg("Profile name character length exceeded")]
    ProfileNameCharLengthExceeded,

    #[msg("URI character length exceeded")]
    URICharLengthExceeded,
}
