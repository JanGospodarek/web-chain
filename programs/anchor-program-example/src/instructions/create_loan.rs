use crate::state::LoanPDA;
use crate::state::Loan;
use anchor_lang::prelude::*;



pub fn create_loan(ctx: Context<CreateLoan>,nft_id:u32,req_amount:u64,interest:u64,period:u64) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    loan.add_loan(Loan{nft_id,req_amount,interest,period,paid_amount:0,borrower:ctx.accounts.payer.key().clone(),lender:Pubkey::default()});
    Ok(())
}


#[derive(Accounts)]
pub struct CreateLoan<'info> {
   #[account(mut)]
    payer: Signer<'info>,

    #[account(mut,seeds = [b"loan_seed",payer.key().as_ref()],bump)]
    loan: Account<'info,LoanPDA>,
    system_program: Program<'info, System>,
}