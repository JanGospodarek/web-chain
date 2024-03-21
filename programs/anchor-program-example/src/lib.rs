#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3WAygPLbRDLygztwB87McLbsUhGgJuvey17k9QHrAwHE");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_page_visits(ctx: Context<CreatePageVisits>,amount:u32,nftId:u32,req_amount:u64,interest:u64,period:u64) -> Result<()> {
        create::create_page_visits(ctx,amount,nftId,req_amount,interest,period)
    }

    pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
        increment::increment_page_visits(ctx)
    }
}
