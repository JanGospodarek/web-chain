use anchor_lang::prelude::*;

use crate::state::LoanPDA;
use crate::state::UserInfo;


pub fn init(ctx: Context<Initialize>) -> Result<()> {
    
    ctx.accounts.loan.bump = ctx.bumps.loan;

    ctx.accounts.user_info.trust_score = 100;
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

    #[account(
        init,    
        space = 8 +UserInfo::INIT_SPACE,
        payer = payer,
        seeds = [b"user_info_seed",payer.key().as_ref()],
        bump,
    )]
    user_info:Account<'info,UserInfo>,
    system_program: Program<'info, System>,
}
