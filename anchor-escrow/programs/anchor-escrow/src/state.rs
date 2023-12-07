use anchor_lang::prelude::*;
use anchor_lang::{Space,solana_program::pubkey::Pubkey};

#[account]
pub struct Escrow{
     pub seed:u64,
     pub mint_a:Pubkey,
     pub mint_b:Pubkey,
     pub receive:u64,
     pub bump:u8,
     pub vault_bump:u8
}

impl Space for Escrow{
    const INIT_SPACE:usize=8+8+32+32+8+1+1;
}