use crate::state::NFTLoan;
use anchor_lang::prelude::*;


pub fn create_nft_loan(ctx: Context<CreateLoan>,nftId:u32,req_amount:u64,interest:u64,period:u64,seed_prefix:String) -> Result<()> {
    *ctx.accounts.loan =NFTLoan {
        nftId: nftId,
        req_amount:req_amount,
        interest:interest,
        period:period,
        bump: ctx.bumps.loan,
    };

    Ok(())
}

#[derive(Accounts)]
#[instruction(seed_prefix:String)]

pub struct CreateLoan<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 +NFTLoan::INIT_SPACE,
        payer = payer,
        seeds = [
           seed_prefix.as_ref(),
            payer.key().as_ref(),
        ],
        bump,
    )]
    loan: Account<'info,NFTLoan>,
    system_program: Program<'info, System>,
}
