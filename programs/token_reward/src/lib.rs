use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

declare_id!("E9yxiWvqXm76keKtxZ5MFQpdpJfDJThgvX85tpVWdzPQ");

#[program]
pub mod token_reward {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<MintTokenPDA>, token_name: String, token_amount: u64) -> Result<()> {
        let token_data = &mut ctx.accounts.token_mint;
        token_data.token_name = token_name;
        token_data.token_amount = token_amount;
        msg!("token has been Created!");
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        let cpi_account = MintTo{
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokenPDA<'info> {
    #[account(
        init,
        seeds = [payer.key().as_ref()],
        bump,
        payer = payer,
        space = 1000,
    )]
    pub token_mint: Account<'info, MyToken>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct MyToken {
    token_name: String,
    token_amount: u64,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: payer account (authority) 
    #[account(mut)]
    pub payer: Signer<'info>,
}