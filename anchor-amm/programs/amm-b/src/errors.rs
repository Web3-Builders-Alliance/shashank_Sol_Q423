use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Fee should be less than 10000(100%).")]
    SlippageExceeded
}

// #[msg("Fee should be less than 10000(100%).")]
// InvalidFee