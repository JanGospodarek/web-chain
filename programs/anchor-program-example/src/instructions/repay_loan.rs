use crate::state::*;
use anchor_lang::prelude::*;
pub fn repay_loan(ctx: Context<RepayLoan>,loan_id:u32,amount:u64) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    loan.repay(loan_id,amount);
    Ok(())
}


#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut,seeds = [b"loan_seed",payer.key().as_ref()],bump)]
    loan: Account<'info,LoanPDA>,
    system_program: Program<'info, System>,
}


