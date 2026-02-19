use crate::state::UserAccount;
use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

#[derive(Accounts)]
pub struct RequestRandomness<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Oracle queue account, validated by the VRF program
    #[account(mut)]
    pub oracle: AccountInfo<'info>,
    #[account(seeds = [b"user", signer.key().as_ref()], bump)]
    pub user: Account<'info, UserAccount>,
    /// CHECK: VRF program
    pub vrf_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestRandomness<'info> {
    pub fn request(&mut self) -> Result<()> {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.signer.key(),
            oracle_queue: self.oracle.key(),
            callback_program_id: crate::ID,
            callback_discriminator: [119, 74, 172, 38, 89, 193, 194, 98].to_vec(),
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.user.key(),
                is_signer: false,
                is_writable: true,
            }]),
            caller_seed: [0u8; 32],
            ..Default::default()
        });

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.signer.to_account_info(),
                self.oracle.to_account_info(),
                self.vrf_program.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}