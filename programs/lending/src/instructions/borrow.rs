use std::f32::consts::E;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::ErrorCode, Bank, User, MAX_AGE, SOL_USD_FEED_ID, USDC_USD_FEED_ID};

#[derive(Accounts)]
pub struct Borrow<'info> {
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

    pub price_update: Account<'info, PriceUpdateV2>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler_borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    let user = &mut ctx.accounts.user_account;

    // Handles token borrowing from the user, calculating and assigning proportional shares to the bank
    let price_update = &mut ctx.accounts.price_update;

    let total_collateral: u64;

    // To check what asset the user needs for collateral
    match ctx.accounts.mint.to_account_info().key() {
        key if key == user.usdc_address => {
            let sol_feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
            let sol_price =
                price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &sol_feed_id)?;
            let new_value = calculate_accrued_interest(
                user.deposited_sol,
                bank.interest_rate,
                user.last_updated,
            )?;
            total_collateral = sol_price.price as u64 * new_value;
        }
        _ => {
            let usdc_feed_id = get_feed_id_from_hex(USDC_USD_FEED_ID)?;
            let usdc_price =
                price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &usdc_feed_id)?;
            let new_value = calculate_accrued_interest(
                user.deposited_usdc,
                bank.interest_rate,
                user.last_updated,
            )?;
            total_collateral = usdc_price.price as u64 * new_value;
        }
    }

    let borrowable_amount = total_collateral
        .checked_mul(bank.liquidation_threshold)
        .unwrap();

    if borrowable_amount < amount {
        return Err(ErrorCode::OverBorrowableAmount.into()); // Borrowing amount exceeds collateral
    }

    // Make transfer to the user
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.bank_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.bank_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury", mint_key.as_ref(), &[bank.treasury_bump]]];

    let cpi_context = CpiContext::new_with_signer(cpi_program, transfer_cpi_accounts, signer_seeds);
    let decimals = ctx.accounts.mint.decimals;

    // Transfer the borrowed tokens to the user's account
    transfer_checked(cpi_context, amount, decimals)?;

    // Update the state of the user and bank
    if bank.total_borrowed == 0 {
        bank.total_borrowed = amount;
        bank.total_borrowed_shares = amount;
    }

    let borrow_ratio = amount.checked_div(bank.total_borrowed).unwrap();
    let user_shares = bank
        .total_borrowed_shares
        .checked_mul(borrow_ratio)
        .unwrap();

    match ctx.accounts.mint.to_account_info().key() {
        key if key == user.usdc_address => {
            user.borrowed_usdc += amount;
            user.borrowed_usdc_shares += user_shares;
        }
        _ => {
            user.borrowed_sol += amount;
            user.borrowed_sol_shares += user_shares;
        }
    }

    user.last_updated_borrowed = Clock::get()?.unix_timestamp;

    Ok(())
}

pub fn calculate_accrued_interest(
    deposited: u64,
    interest_rate: u64,
    last_updated: i64,
) -> Result<u64> {
    let current_time = Clock::get()?.unix_timestamp;
    let time_difference = current_time - last_updated;
    let new_value =
        (deposited as f64 * E.powf(interest_rate as f32 * time_difference as f32) as f64) as u64;

    Ok(new_value)
}
