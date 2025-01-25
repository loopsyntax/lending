use std::f64::consts::E;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::error::ErrorCode;
use crate::{Bank, User};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump = bank.bank_bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"treasury", mint.key().as_ref()],
        bump = bank.treasury_bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let bank = &mut ctx.accounts.bank;

    // Handles token deposits into the bank, calculating and assigning proportional shares to depositor
    let deposited_value: u64;
    if ctx.accounts.mint.to_account_info().key() == user.usdc_address {
        deposited_value = user.deposited_usdc;
    } else {
        deposited_value = user.deposited_sol;
    }

    let time_difference = user.last_updated - Clock::get()?.unix_timestamp;

    // Validates withdrawal amount against user's deposit value with accrued interest over time
    bank.total_deposits = (bank.total_deposits as f64
        * E.powf(bank.interest_rate as f64 * time_difference as f64))
        as u64;

    let value_per_share = bank.total_deposits as f64 / bank.total_deposit_shares as f64;
    let user_value = deposited_value as f64 / value_per_share;

    if user_value < amount as f64 {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    // if amount > deposited_value {
    //     return Err(ErrorCode::InsufficientFunds.into());
    // }

    // This handles the withdrawal of tokens
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.bank_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let mint_key = ctx.accounts.mint.key();

    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", mint_key.as_ref(), &[bank.treasury_bump]]];

    let cpi_context = CpiContext::new_with_signer(cpi_program, transfer_cpi_accounts, signer_seeds);
    let decimals = ctx.accounts.mint.decimals;

    transfer_checked(cpi_context, amount, decimals)?;

    // Update the user's state and handles the accounting
    let shares_to_remove =
        (amount as f64 / bank.total_deposits as f64) * bank.total_deposit_shares as f64;

    if ctx.accounts.mint.to_account_info().key() == user.usdc_address {
        user.deposited_usdc -= amount;
        user.deposited_usdc_shares -= shares_to_remove as u64;
    } else {
        user.deposited_sol -= amount;
        user.deposited_sol_shares -= shares_to_remove as u64;
    }

    bank.total_deposits -= amount;
    bank.total_deposit_shares -= shares_to_remove as u64;

    Ok(())
}
