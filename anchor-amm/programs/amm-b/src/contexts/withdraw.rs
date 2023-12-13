use anchor_lang::prelude::*;

use anchor_spl::{token_interface::{Mint, TokenInterface, TokenAccount,Transfer,transfer_checked}, associated_token::AssociatedToken};
use constant_product_curve::ConstantProduct;
use crate::{state::Config, assert_not_expired, assert_non_zero, accounts};
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Withdraw {
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

impl<'info> Withdraw<'info> {
  pub fn withdraw(
      &mut self, 
      amount:u64, // amount of lp tokens to create/mint/claim
      min_x:u64, //max amount of x we are willing to deposit
      max_y:u64, //max amount of y we are willing to deposit
      expiration:i64
  ) -> Result<()> {
      // assert_not_expired!(expiration)?;
      assert_not_expired!(expiration)?;
      assert_non_zero!([amount])?;
      // assert_non_zero!([amount])?;

      let (x, y) = match 
      self.mint_lp.supply == 0 && 
      self.vault_x.amount == 0 && 
      self.vault_y.amount == 0 {
          true=> (min_x, max_y),
          false=> {
            // ConstantProduct
              let ammounts = ConstantProduct::xy_deposit_amounts_from_l(
                  self.vault_x.amount,
                  self.vault_y.amount,
                  self.mint_lp.supply,
                  amount,
                  self.mint_lp.decimals
              ).map_err(AmmError::from)?;
          }
      };

      require!(min_x <= amount.y,AmmError::SlippageExceeded);
      unimplemented!()
      // umimplemented!();
  }

  pub fn withdraw_tokens(&self, is_x:bool, amount:u64) -> Result<()> {
      let (from, to, mint, decimals) = match is_x {
          true=> (self.vault_x.to_account_info(),self.user_x.to_account_info(),self.mint_x.to_account_info(), self.mint_x.decimals),
          false=> (self.vault_y.to_account_info(),self.user_y.to_account_info(), self.mint_y.to_account_info(), self.mint_y.decimals)
      };

      let seeds = &[
        &b"auth"[..],
        &[self.config.auth_bump],
    ];
      let signer_seeds = &[&seeds[..]];


      let transfer_instruction=CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        Transfer {
            from,
            to,
            authority: self.auth.to_account_info(),
        },
        &[self.auth.to_account_info()],
    );

      transfer_checked(transfer_instruction, amount, decimals);
      Ok(())
  }


}
