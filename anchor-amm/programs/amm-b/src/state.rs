use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed:u64,
    pub authority:Option<Pubkey>,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub fee:u16,
    pub auth_bump:u8,
    pub lp_bump:u8,
    pub bump:u8,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 8 + 32 * 2 + 2 + 1 * 3;
}

impl Config {
    pub fn init(
        &mut self,
        seed:u64,
        authority:Option<Pubkey>,
        mint_x:Pubkey,
        mint_y:Pubkey,
        fee:u16,
        auth_bump:u8,
        bump:u8,
        lp_bump:u8
    ) -> Result<()> {
        self.seed = seed;
        self.authority = authority;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.fee = fee;
        self.auth_bump = auth_bump;
        self.bump = bump;
        self.lp_bump = bump;

        Ok(())
    }
}
