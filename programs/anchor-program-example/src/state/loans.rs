use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace,PartialEq)]
pub enum LoanState {
    Acitve,
    Pending,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct Loan {
    pub loan_id: u32,
    pub nft_id: u32,
    pub req_amount: u64,
    pub interest: u64,
    pub period: u64,
    pub paid_amount: u64,
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub state: LoanState,
}   
#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy,InitSpace)]
pub struct HistoryLoan {
    pub loan_id: u32,
    pub nft_id: u32,
    pub lender: Pubkey,
    pub borrower: Pubkey,
}   
#[account]
#[derive(InitSpace)] 
pub struct LoanPDA {
    pub bump: u8,
    pub loans: [Option<Loan>; 10],
    pub history_loans: [Option<HistoryLoan>; 10],
    pub loan_count: u8

}

impl LoanPDA{
    pub fn add_loan(&mut self,loan:Loan)->&str{
        if let Some(index) = self.loans.iter().position(|&x| x.is_none()) {
            self.loans[index] = Some(loan);
            return "success";
        }else{
            return "no-space"
        }
    }
    pub fn destroy_loan(&mut self,loan_id:u32){
        if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
            if self.loans[index].unwrap().state != LoanState::Pending {
                return;
            }
            self.loans[index] = None;
        }
    }

    pub fn set_lender(&mut self,loan_id:u32,lender:Pubkey){
        if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
            if let Some(mut loan) = self.loans[index].take() {
                loan.lender = lender;
                loan.state = LoanState::Acitve;
                self.loans[index] = Some(loan);
            }
        }
    }
    pub fn repay(&mut self,loan_id:u32,amount:u64){
        if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
            if let Some(mut loan) = self.loans[index].take() {
                loan.paid_amount += amount;
                if loan.paid_amount >= loan.req_amount {
                    if let Some(i) = self.loans.iter().position(|&x| x.is_none()){
                        self.history_loans[i] = Some(HistoryLoan{
                            loan_id:loan.loan_id,
                            nft_id:loan.nft_id,
                            lender:loan.lender,
                            borrower:loan.borrower,
                        });
                        self.loans[index] = None;
                    }
                }else{
                    self.loans[index] = Some(loan);

                }
               
            }
        }
    }
}