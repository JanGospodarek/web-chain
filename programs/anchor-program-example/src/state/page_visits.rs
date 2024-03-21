use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct PageVisits {
    pub page_visits: u32,
    pub nftId: u32,
    pub req_amount:u64,
    pub interest:u64,
    pub period:u64,
    pub bump: u8,

}

impl PageVisits {
    pub const SEED_PREFIX: &'static [u8; 11] = b"page_visits";

    pub fn increment(&mut self) {
        self.page_visits = self.page_visits.checked_add(1).unwrap();
    }
}
