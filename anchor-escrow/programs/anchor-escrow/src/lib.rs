use anchor_lang::prelude::*;

declare_id!("BeiFzCRT6F7isRsKQHyTTLARxPNVBnvP5WzFY6K1BRjY");
pub mod contexts;
use contexts::*;
pub mod state;
use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,deposit:u64,receive:u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save_escrow(seed,receive,&ctx.bumps);
        
    }
   pub fn refund(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.refund()?;
    ctx.accounts.close_vault()?; 

    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close_vault();
        // Ok(())
    }

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// pub struct Initialize {}
