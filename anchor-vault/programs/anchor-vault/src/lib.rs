use anchor_lang::prelude::*;

declare_id!("7ufKSoLcSQ43jzndM1wo67RTtZ5nT6wj9oULyfVsBmUt");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
