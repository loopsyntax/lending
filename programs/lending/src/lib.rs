pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6E3EJE7gcpDnhvmx2qFo5SXDNnAFCwyFew8JoTarNXas");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize_bank(
        ctx: Context<InitializeBank>,
        liquidation_threshold: u64,
        max_ltw: u64,
    ) -> Result<()> {
        initialize_bank::handler_initialize_bank(ctx, liquidation_threshold, max_ltw)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, usdc_address: Pubkey) -> Result<()> {
        initialize_user::handler_initialize_user(ctx, usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::handler_deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::handler_withdraw(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        borrow::handler_borrow(ctx, amount)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        repay::handler_repay(ctx, amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        liquidate::handler_liquidate(ctx)
    }
}
