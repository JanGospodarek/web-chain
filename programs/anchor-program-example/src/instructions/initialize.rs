use anchor_lang::prelude::*;

use crate::state::LoanPda;
use crate::state::UserInfo;


pub fn init(ctx: Context<Initialize>) -> Result<()> {
    let loan= &mut ctx.accounts.loan;

    loan.bump = ctx.bumps.loan;
    loan.loan_count = 0;
    loan.loans=[None;10];
    loan.space = LoanPda::INIT_SPACE as u32;
    ctx.accounts.user_info.trust_score = 100;
    ctx.accounts.user_info.owner = *ctx.accounts.payer.key;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 +LoanPda::INIT_SPACE,
        payer = payer,
        seeds = [b"prefix_loan_seed",payer.key().as_ref()],
        bump,
    )]
    loan: Account<'info,LoanPda>,

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
