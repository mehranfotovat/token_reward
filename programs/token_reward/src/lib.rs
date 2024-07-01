use anchor_lang::prelude::*;

declare_id!("E9yxiWvqXm76keKtxZ5MFQpdpJfDJThgvX85tpVWdzPQ");

#[program]
pub mod token_reward {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
