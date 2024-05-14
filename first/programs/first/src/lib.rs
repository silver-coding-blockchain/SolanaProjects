use anchor_lang::prelude::*;

declare_id!("2oGW5uyb4SzDk6FchKs1UZSuB6jrK4NC7XEkFa5Ro8hf");

#[program]
pub mod first {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
