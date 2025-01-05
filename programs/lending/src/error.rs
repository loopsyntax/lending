use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Borrowing amount exceeds collateral")]
    OverBorrowableAmount,
    #[msg("Requested amount exceeds depositable amount")]
    OverRepay,
}
