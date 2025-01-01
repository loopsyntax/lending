use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub authority: Pubkey,
    pub mint_address: Pubkey,
    pub total_deposits: u64,
    pub total_deposit_shares: u64,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub liquidation_close_factor: u64,
    pub max_ltw: u64,
    pub last_updated: u64,
}
