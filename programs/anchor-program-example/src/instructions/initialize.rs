use anchor_lang::prelude::*;

use crate::state::LoanPDA;
use crate::state::UserInfo;


pub fn init(ctx: Context<Initialize>) -> Result<()> {
    let loan= &mut ctx.accounts.loan;

    loan.bump = ctx.bumps.loan;
    loan.loan_count = 0;
    loan.loans=[None;10];
    loan.space = LoanPDA::INIT_SPACE as u32;
    ctx.accounts.user_info.trust_score = 100;

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
