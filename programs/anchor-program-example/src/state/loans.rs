use anchor_lang::prelude::*;
// use borsh::{BorshDeserialize, BorshSerialize};
// use anchor_lang::{AnchorSerialize,AnchorDeserialize};anchor
#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct Loan {
    pub nft_id: u32,
    pub req_amount: u64,
    pub interest: u64,
    pub period: u64,
}

#[account]
#[derive(InitSpace)] 
pub struct LoanPDA {
    pub bump: u8,
    pub loans: [Option<Loan>; 15],
    pub loan_count: u8,

}
