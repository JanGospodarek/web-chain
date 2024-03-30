use crate::state::*;
use anchor_lang::prelude::*;
pub fn repay_loan(ctx: Context<RepayLoan>,loan_id:u32,amount:u64) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let borrower_user_info = &mut ctx.accounts.borrower_user_info;
    let lender_user_info = &mut ctx.accounts.lender_user_info;

    let req_amount=loan.repay(loan_id,amount);
    borrower_user_info.increase_trust_score(req_amount);
    lender_user_info.increase_trust_score(req_amount);
    Ok(())
}


#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    ///CHECK: wow
    borrower: AccountInfo<'info>,
    #[account(mut,seeds = [b"loan_seed",borrower.key().as_ref()],bump)]
    loan: Account<'info,LoanPDA>,

    #[account(mut,seeds = [b"user_info_seed",borrower.key().as_ref()],bump)]
    borrower_user_info: Account<'info,UserInfo>,

    #[account(mut,seeds = [b"user_info_seed",payer.key().as_ref()],bump)]
    lender_user_info: Account<'info,UserInfo>,
    system_program: Program<'info, System>,
}


