#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3WAygPLbRDLygztwB87McLbsUhGgJuvey17k9QHrAwHE");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn init(ctx: Context<Initialize>) -> Result<()> {
        initialize::init(ctx)
    }
    pub fn create_nft_loan(ctx: Context<CreateLoan>,nft_id:u32,req_amount:u64,interest:u64,period:u64) -> Result<()> {
     let res= create_loan::create_loan(ctx,nft_id,req_amount,interest,period);
     
        match res {
            Ok(_)=>Ok(()),
            Err(e)=>Err(e)
        }

    }
    pub fn destroy_loan(ctx: Context<DestroyLoan>,nft_id:u32) -> Result<()> {
        destroy_loan::destroy_loan(ctx,nft_id)
    }

    // pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
    //     increment::increment_page_visits(ctx)
    // }
}
