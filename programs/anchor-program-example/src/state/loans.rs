use anchor_lang::prelude::*;

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

impl LoanPDA{
    pub fn add_loan(&mut self,loan:Loan){
        let loan_count = self.loan_count;
        if loan_count<15 {
            self.loans[loan_count as usize]=Some(loan);
            self.loan_count+=1;
        }

    }
}