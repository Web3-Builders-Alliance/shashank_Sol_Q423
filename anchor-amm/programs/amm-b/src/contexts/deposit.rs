use anchor_lang::prelude::*;
#[macro_use]
use anchor_spl::{token_interface::{Mint, TokenInterface, TokenAccount, mint_to, MintTo, TransferChecked, transfer_checked}, associated_token::AssociatedToken};
use constant_product_curve::ConstantProduct;
use crate::{state::Config, assert_not_expired, assert_non_zero};
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Deposit {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"lp", config.key.as_ref()], 
        bump=config.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: this is just used for sigming.
    #[account(seeds = [b"auth", config.key.as_ref()], bump=config.auth_bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &mut self, 
        amount:u64, // amount of lp tokens to create/mint/claim
        max_x:u64, //max amount of x we are willing to deposit
        max_y:u64, //max amount of y we are willing to deposit
        expiration:i64
    ) -> Result<()> {
        assert_not_expired!(expiration)?;
        // assert_non_zero!()
        assert_non_zero!([amount, max_x, max_y])?;

        let (x, y) = match 
        self.mint_lp.supply == 0 && 
        self.vault_x.amount == 0 && 
        self.vault_y.amount == 0 {
            true=> (max_x, max_y),
            false=> {
                let ammounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    0
                ).map_err(AmmError::from)?;
                (ammounts.x, ammounts.y)
            }
        };

        //check for slippage
        require!(x <= max_x, y <= max_y, "Slippage Exceeded")?;
        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp_tokens(amount)
    }

    pub fn deposit_tokens(&self, is_x:bool, amount:u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true=> (self.user_x.to_account_info(), self.vault_x.to_account_info(), self.mint_x.to_account_info(), self.mint_x.decimals),
            false=> (self.user_y.to_account_info(), self.vault_y.to_account_info(), self.mint_y.to_account_info(), self.mint_y.decimals)
        };
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.auth.to_account_info()
        };
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            accounts
        );

        transfer_checked(cpi_ctx, amount, decimals)
    }

    pub fn mint_lp_tokens(&self, amount:u64) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let seeds = [&[
            b"auth",
            self.config.key().as_ref(),
            &[self.cofig.auth_bump],
        ]];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );
        mint_to(cpi_ctx, amount)
    }
}
