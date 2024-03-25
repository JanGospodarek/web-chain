use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct UserInfo{
    pub trust_score:u32,
}