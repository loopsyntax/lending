use anchor_lang::prelude::*;

use crate::{User, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + User::INIT_SPACE,
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

pub fn handler_initialize_user(ctx: Context<InitializeUser>, usdc_address: Pubkey) -> Result<()> {
    ctx.accounts.user_account.set_inner(User {
        owner: ctx.accounts.signer.key(),
        deposited_sol: 0,
        deposited_sol_shares: 0,
        borrowed_sol: 0,
        borrowed_sol_shares: 0,
        deposited_usdc: 0,
        deposited_usdc_shares: 0,
        borrowed_usdc: 0,
        borrowed_usdc_shares: 0,
        usdc_address,
        health_factor: 0,
        last_updated: 0,
        last_updated_borrow: 0,
    });
    Ok(())
}
