// use crate::state::NFTLoan;
// use anchor_lang::prelude::*;

// #[derive(Accounts)]
// pub struct IncrementPageVisits<'info> {
//     user: SystemAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             NFTLoan::SEED_PREFIX,
//             user.key().as_ref(),
//         ],
//         bump = loan.bump,
//     )]
//     loan: Account<'info, NFTLoan>,
// }

// pub fn increment_page_visits(ctx: Context<IncrementPageVisits>) -> Result<()> {
//     let loan = &mut ctx.accounts.loan;
//     loan.increment();
//     Ok(())
// }
