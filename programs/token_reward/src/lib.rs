use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo, Transfer};

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
        // let seed = ctx.accounts.payer.key();
        // let bump_seed = ctx.bumps.token_pda;
        // let bump_vector = bump_seed.to_le_bytes();
        // let inner = vec![seed.as_ref(), &bump_vector];
        // let outer = vec![inner.as_slice()];
        let seed = ctx.accounts.payer.key();
        let bump_seed = ctx.bumps.token_pda;
        let signer: &[&[&[u8]]] = &[&[seed.as_ref(), &[bump_seed]]];

        let cpi_account = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.token_pda.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account).with_signer(signer);

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64, bump: u64) -> Result<()> {
        //create signer seed
        let seed = ctx.accounts.payer.key();
        let bump_seed = bump as u8;
        let signer: &[&[&[u8]]] = &[&[seed.as_ref(), &[bump_seed]]];
        // Create the Transfer struct for our context
        let transfer_instruction = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.token_pda.to_account_info(),
        };
        // token program account
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the Context for our Transfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction).with_signer(signer);

        // Execute anchor's helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, amount)?;
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
    /// CHECK: (authority pda)
    #[account(
        seeds = [payer.key().as_ref()],
        bump,
    )]
    pub token_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    /// CHECK: (authority pda)
    #[account(mut)]
    pub token_pda: AccountInfo<'info>,
    /// CHECK: payer account (authority) 
    #[account(mut)]
    pub payer: Signer<'info>,
}
