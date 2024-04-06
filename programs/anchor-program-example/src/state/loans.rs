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
    pub period: i64,
    pub paid_amount: u64,
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub state: LoanState,
}   

#[account]
#[derive(InitSpace)] 
pub struct LoanPda {
    pub bump: u8,
    pub loans: [Option<Loan>; 10],
    pub space:u32,
    pub loan_count: u8

}

impl LoanPda{
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
            if self.loans[index].unwrap().state == LoanState::Acitve {
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
    pub fn repay(&mut self,loan_id:u32,amount:u64)->u64{

        if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
            if let Some(mut loan) = self.loans[index].take() {
                loan.paid_amount += amount;
                let mut amount = 0;

                let multiplyer: f64 = loan.interest as f64 / 100.0 + 1.0;
                let total=(loan.req_amount as f64) *multiplyer;

                if loan.paid_amount as f64 >= total {
                    loan.state = LoanState::Closed;
                    amount=loan.req_amount
                }

                self.loans[index] = Some(loan);
                return amount;
      
            }
        }
        return 0;
        
        
    }
    // pub fn buy_a_loan(&mut self,loan_id:u32,buyer:Pubkey,new_duration:i64){
    //     if let Some(index) = self.loans.iter().position(|&x| x.is_some() && x.unwrap().loan_id == loan_id) {
    //         if let Some(mut loan) = self.loans[index].take() {
    //             loan.lender = buyer;
    //             lo
    //             self.loans[index] = Some(loan);
    //         }
    //     }
    // }
}