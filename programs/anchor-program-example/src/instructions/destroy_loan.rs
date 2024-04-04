use crate::state::LoanPda;
use anchor_lang::prelude::*;



pub fn destroy_loan(ctx: Context<DestroyLoan>,loan_id:u32) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    loan.destroy_loan(loan_id);
    Ok(())
}


#[derive(Accounts)]
pub struct DestroyLoan<'info> {
   #[account(mut)]
    payer: Signer<'info>,

    #[account(mut,seeds = [b"prefix_loan_seed",payer.key().as_ref()],bump)]
    loan: Account<'info,LoanPda>,
    system_program: Program<'info, System>,
}
