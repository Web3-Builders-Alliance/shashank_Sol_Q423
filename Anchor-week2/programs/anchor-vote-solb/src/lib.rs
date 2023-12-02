    use anchor_lang::{prelude::*,solana_program::hash::hash};

declare_id!("vote7hbbvFUD8KoUcoZEcS57aMpqZFYQiCLJuJCVLZm");

#[program]
pub mod anchor_vote_solb {
    

    use super::*;

    pub fn initialize(ctx: Context<Initialize>,_url:String) -> Result<()> {
        ctx.accounts.vote.score=0;
        ctx.accounts.vote.bump=ctx.bumps.vote;
        // let sha= hash(url.as_bytes()).to_bytes();
        Ok(())
    }
    pub fn upvote(ctx: Context<Vote>) -> Result<()> {
        ctx.accounts.vote.score +=1;
        Ok(())
    }
    pub fn downvote(ctx: Context<Vote>) -> Result<()> {
        ctx.accounts.vote.score -=1;
        Ok(())
    }        
}

#[derive(Accounts)]
#[instruction(_url:String)]
pub struct Initialize<'info> {
   #[account(mut)]
    signer:Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=VoteState::INIT_SPACE,
        seeds=[hash(_url.as_bytes()).to_bytes().as_ref()],
        bump

    )]
    vote:Account<'info,VoteState>,
    system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(_url:String)]
pub struct Vote<'info> {
   #[account(mut)]
    signer:Signer<'info>,
    #[account(
        mut,
        seeds=[hash(_url.as_bytes()).to_bytes().as_ref()],
        bump = vote.bump

    )]
    vote:Account<'info,VoteState>,
    system_program:Program<'info,System>
}

#[account]
pub struct VoteState{
   score:i64,
   bump:u8
}

// sha256("global:Vote")[0..8]

impl Space for VoteState{
    const INIT_SPACE: usize = 8 + 8 + 1;
}