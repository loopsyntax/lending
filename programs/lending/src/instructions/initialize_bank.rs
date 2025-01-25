use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{Bank, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializeBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + Bank::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        token::mint = mint,
        token::authority = bank_token_account,
        payer = signer,
        seeds = [b"treasury", mint.key().as_ref()],
        bump,
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler_initialize_bank(
    ctx: Context<InitializeBank>,
    liquidation_threshold: u64,
    max_ltw: u64,
) -> Result<()> {
    ctx.accounts.bank.set_inner(Bank {
        authority: ctx.accounts.signer.key(),
        mint_address: ctx.accounts.mint.key(),
        total_deposits: 0,
        total_deposit_shares: 0,
        liquidation_threshold,
        liquidation_bonus: 0,
        liquidation_close_factor: 0,
        max_ltw,
        last_updated: 0,
        interest_rate: 0.05 as u64,
        total_borrowed: 0,
        total_borrowed_shares: 0,
        bank_bump: ctx.bumps.bank,
        treasury_bump: ctx.bumps.bank_token_account,
    });

    Ok(())
}
