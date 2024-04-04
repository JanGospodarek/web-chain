use crate::state::*;
use anchor_lang::prelude::*;
pub fn accept_offer(ctx: Context<AcceptOffer>,loan_id:u32) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let lender = ctx.accounts.payer.key().clone();

    loan.set_lender(loan_id,lender);
    Ok(())
}


#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(mut)]
    ///CHECK: wow
    borrower: AccountInfo<'info>,
    #[account(mut,seeds = [b"prefix_loan_seed",borrower.key().as_ref()],bump)]
    loan: Account<'info,LoanPda>,
    system_program: Program<'info, System>,
}


