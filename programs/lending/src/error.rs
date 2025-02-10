use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Borrowing amount exceeds collateral")]
    OverBorrowableAmount,
    #[msg("Requested amount exceeds depositable amount")]
    OverRepay,
    #[msg("User is not under collaterized, can't be liquidated")]
    NotUnderCollaterized,
    #[msg("No outstanding borrows")]
    NoOutstandingBorrows,
    #[msg("Math Overflow")]
    MathOverflow,
    #[msg("No deposits")]
    NoDeposits,
}
