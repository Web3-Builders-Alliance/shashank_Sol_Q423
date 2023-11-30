use anchor_lang::prelude::*;

declare_id!("9VSCLy3hTLydi6igAGVbDE2yPapv1t4PW4QH7btBsj26");

#[program]
pub mod anchor_vote_solb {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,url:String) -> Result<()> {
        Ok(())
    }
    pub fn upvote(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn downvote(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }        
}

#[derive(Accounts)]
pub struct Initialize<'info> {
   #[account(mut)]
    signer:Signer<'info>,
    vote:Account<'info,Vote>
}

#[account]
pub struct Vote{
   score:i64
}

impl Space for Vote{
    const INIT_SPACE: usize = 8 + 8;
}