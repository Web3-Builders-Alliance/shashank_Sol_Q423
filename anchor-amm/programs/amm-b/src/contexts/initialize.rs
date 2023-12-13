use anchor_lang::prelude::*;

use anchor_spl::{token_interface::{Mint, TokenInterface, TokenAccount}, associated_token::AssociatedToken};
use crate::state::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init, 
        payer=payer, 
        seeds = [b"lp", config.key.as_ref()], 
        bump, 
        mint::decimals = 6, 
        mint::authority = auth
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=payer,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=payer,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: this is just used for sigming.
    #[account(seeds = [b"auth", config.key.as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer=payer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(fee <= 10000, AmmError::InvalidFee);
        self.config.init(seed, authority, mint_x, mint_y, fee, bumps.auth, bumps.config, bumps.mint_lp)
    }
}
