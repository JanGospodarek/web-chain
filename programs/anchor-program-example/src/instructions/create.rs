use crate::state::PageVisits;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreatePageVisits<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 + PageVisits::INIT_SPACE,
        payer = payer,
        seeds = [
            PageVisits::SEED_PREFIX,
            payer.key().as_ref(),
        ],
        bump,
    )]
    page_visits: Account<'info, PageVisits>,
    system_program: Program<'info, System>,
}

pub fn create_page_visits(ctx: Context<CreatePageVisits>,amount:u32,nftId:u32,req_amount:u64,interest:u64,period:u64) -> Result<()> {
    *ctx.accounts.page_visits = PageVisits {
        page_visits: amount,
        nftId: nftId,
        req_amount:req_amount,
        interest:interest,
        period:period,
        bump: ctx.bumps.page_visits,
    };

    Ok(())
}
