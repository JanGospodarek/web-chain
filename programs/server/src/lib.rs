use anchor_lang::prelude::*;

declare_id!("HM6FUz4chhAu135A57DNL9rvY2qvHpq5d78CozbgaUWv");

#[program]
pub mod server {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
