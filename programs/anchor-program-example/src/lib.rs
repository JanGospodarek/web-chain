#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("4MrxHjTHXpjwmnwLPpLjFduTePpazMExe5epoM1gDeDR");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn init(ctx: Context<Initialize>) -> Result<()> {
        initialize::init(ctx)
    }
    pub fn create_nft_loan(ctx: Context<CreateLoan>,loan_id:u32,nft_id:u32,req_amount:u64,interest:u64,period:i64) -> Result<()> {
     let res= create_loan::create_loan(ctx,loan_id,nft_id,req_amount,interest,period);
     
        match res {
            Ok(_)=>Ok(()),
            Err(e)=>Err(e)
        }

    }
    pub fn destroy_loan(ctx: Context<DestroyLoan>,loan_id:u32) -> Result<()> {
        destroy_loan::destroy_loan(ctx,loan_id)
    }
    pub fn accept_offer(ctx: Context<AcceptOffer>,loan_id:u32) -> Result<()> {
        accept_offer::accept_offer(ctx,loan_id)
    }
    pub fn repay_loan(ctx: Context<RepayLoan>,loan_id:u32,amount:u64) -> Result<()> {
        repay_loan::repay_loan(ctx,loan_id,amount)
    }
  
}
