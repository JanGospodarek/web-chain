use crate::state::*;
use anchor_lang::prelude::*;
// use rand::prelude::*;    

pub fn create_loan(ctx: Context<CreateLoan>,loan_id:u32, nft_id: u32, req_amount: u64, interest: u64, period: u64) -> Result<()> {
    let loan = &mut ctx.accounts.loan;

    // dummy id generator
    // let mut rng = rand::thread_rng();
    // let loan_id = rng.gen_range(0..1000) as u64;
    let status = loan.add_loan(Loan {
        loan_id,
        nft_id,
        req_amount,
        interest,
        period,
        paid_amount: 0,
        borrower: ctx.accounts.payer.key().clone(),
        lender: Pubkey::default(),
        state: LoanState::Pending,
    });
    
    if status == "no-space" {
        Err(ErrorCode::DataTooLarge.into()) 
    } else {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CreateLoan<'info> {
   #[account(mut)]
    payer: Signer<'info>,

    #[account(mut,seeds = [b"loan_seed",payer.key().as_ref()],bump)]
    loan: Account<'info,LoanPDA>,
    system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}