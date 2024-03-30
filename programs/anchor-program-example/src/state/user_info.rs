use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct UserInfo{
    pub trust_score:u32,
}

impl UserInfo  {
   pub fn increase_trust_score(&mut self,loan_amount:u64){
    let increase = ((loan_amount as f64 / 1000000.0) * 10.0).ceil() as u32;
        self.trust_score += increase;
    }
}