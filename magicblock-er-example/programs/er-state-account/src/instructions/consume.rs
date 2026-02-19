use crate::state::UserAccount;
use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::rnd::random_u64;

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    #[account(mut)]
    pub user: Account<'info, UserAccount>,
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_ID)]
    pub program: Signer<'info>,
}

impl<'info> ConsumeRandomness<'info> {
    pub fn consume_random(&mut self, randomness: [u8; 32]) -> Result<()> {
        let rnd_u64 = random_u64(&randomness);
        self.user.data = rnd_u64;
        msg!("User data updated with random value: {}", rnd_u64);
        Ok(())
    }
}