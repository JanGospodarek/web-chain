use crate::state::LoanPDA;
use anchor_lang::prelude::*;


pub fn init(ctx: Context<Initialize>) -> Result<()> {
    
    ctx.accounts.loan.bump = ctx.bumps.loan;
    ctx.accounts.loan.loan_count = 0;
    ctx.accounts.loan.loans=[None;10];
  
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 +LoanPDA::INIT_SPACE,
        payer = payer,
        seeds = [b"loan_seed",payer.key().as_ref()],
        bump,
    )]
    loan: Account<'info,LoanPDA>,
    
    system_program: Program<'info, System>,
}
