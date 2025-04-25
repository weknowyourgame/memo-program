use anchor_lang::prelude::*;

declare_id!("HSwTZnfD7aDYVnuKQeNXBNNA71xDgWXMWMwkLbsJCxmr");

#[program]
pub mod memo_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
