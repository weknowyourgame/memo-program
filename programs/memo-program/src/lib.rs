use anchor_lang::prelude::*;

declare_id!("HSwTZnfD7aDYVnuKQeNXBNNA71xDgWXMWMwkLbsJCxmr");

pub use {
    solana_account_info::AccountInfo, solana_program_entrypoint::ProgramResult,
    solana_pubkey::Pubkey,
}

#[program]
pub mod memo_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
