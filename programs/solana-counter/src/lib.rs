use anchor_lang::prelude::*;

declare_id!("C1kF876QutJ1jAU8grXRy5H7ZiHnS5rW6nhYMRaWyfDJ");

#[program]
pub mod solana_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
