    use anchor_lang::prelude::*;

declare_id!("voteLp9rMPNKRRWwt7Eya81zyFCjzwk2T5XYRJQYtLf");

#[program]
pub mod anchor_vault {
    use anchor_lang::system_program::{Transfer,transfer};

    use super::*;

    pub fn deposit(ctx: Context<Vault>,lamports:u64) -> Result<()> {
        let accounts=Transfer{
            from:ctx.accounts.signer.to_account_info(),
            to:ctx.accounts.vault.to_account_info()
        };
        // let cpi_ctx1=CpiContext::new(program, accounts);

        let cpi_ctx=CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            accounts
        );
        transfer(cpi_ctx,lamports)
        // Ok(())
    }

    pub fn close(ctx: Context<Vault>) -> Result<()> {
        let accounts: Transfer<'_> =Transfer{
            to:ctx.accounts.signer.to_account_info(),
            from:ctx.accounts.vault.to_account_info()
        };

        msg!("log something");
        let binding = ctx.accounts.signer.clone().key();
        let signer_seeds:[&[&[u8]];1]=[&[b"vault",&binding.as_ref(),&[ctx.bumps.vault]]];
        // let cpi_ctx =CpiContext::new_with_signer(program:ctx.accounts.system_program.to_account_info(), accounts);

        let cpi_ctx=CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            accounts,
            &signer_seeds
        );

        transfer(cpi_ctx,ctx.accounts.vault.lamports())
        // Ok(())
    }    
}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault",signer.key().as_ref()],
        bump
    )]
    vault:SystemAccount<'info>,
    system_program:Program<'info,System>
}
