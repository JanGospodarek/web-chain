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
        println!("Init");
        initialize::init(ctx)
    }
    pub fn create_nft_loan(ctx: Context<CreateLoan>,nft_id:u32,req_amount:u64,interest:u64,period:u64) -> Result<()> {
        create::create_loan(ctx,nft_id,req_amount,interest,period)
    }

    // pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
    //     increment::increment_page_visits(ctx)
    // }
}
