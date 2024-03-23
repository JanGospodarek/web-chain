use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct Loan {
    pub nft_id: u32,
    pub req_amount: u64,
    pub interest: u64,
    pub period: u64,
    pub paid_amount: u64,
    pub lender: Pubkey,
    pub borrower: Pubkey,
}

#[account]
#[derive(InitSpace)] 
pub struct LoanPDA {
    pub bump: u8,
    pub loans: [Option<Loan>; 10],
    pub loan_count: u8,

}

impl LoanPDA{
    pub fn add_loan(&mut self,loan:Loan){
        if let Some(index) = self.loans.iter().position(|&x| x.is_none()) {
            self.loans[index] = Some(loan);
            self.loan_count += 1;
        }

    }
}