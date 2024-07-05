use anchor_lang::prelude::*;

declare_id!("E9yxiWvqXm76keKtxZ5MFQpdpJfDJThgvX85tpVWdzPQ");

#[program]
pub mod token_reward {
    use super::*;

    pub fn initialize(ctx: Context<MintToken>, token_name: String, token_amount: u64) -> Result<()> {
        let token_data = &mut ctx.accounts.token_mint;
        token_data.token_name = token_name;
        token_data.token_amount = token_amount;
        msg!("token has been Created!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
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