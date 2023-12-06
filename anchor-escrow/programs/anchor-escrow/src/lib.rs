use anchor_lang::prelude::*;

declare_id!("BeiFzCRT6F7isRsKQHyTTLARxPNVBnvP5WzFY6K1BRjY");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
