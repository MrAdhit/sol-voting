use anchor_lang::prelude::*;

declare_id!("3DZZerpFUiu9kvANNhhAs7h8qUTEvghwgCfm9Me8of6U");

#[program]
pub mod voting {
    use std::ops::DerefMut;

    use super::*;

    pub fn create_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        poll_name: String,
        description: String,
        end_time: u64,
    ) -> Result<()> {
        *(&mut ctx.accounts.poll_account).deref_mut() = PollAccount {
            poll_id,
            poll_name,
            description,
            poll_voting_start: Clock::get()?.unix_timestamp as u64,
            poll_voting_end: end_time,
        };

        Ok(())
    }
    
    pub fn create_candidate(
        ctx: Context<InitializeCandidate>,
        poll_id: u64,
        candidate_name: String,
    ) -> Result<()> {
        *(&mut ctx.accounts.candidate_account).deref_mut() = CandidateAccount {
            poll_id,
            candidate_name,
            candidate_votes: 0,
        };

        Ok(())
    }
    
    pub fn vote(
        ctx: Context<Vote>,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        if current_time > ctx.accounts.poll_account.poll_voting_end as _ {
            return Err(ProgramError::NonVotingPeriod.into());
        }

        if current_time < ctx.accounts.poll_account.poll_voting_start as _ {
            return Err(ProgramError::NonVotingPeriod.into());
        }
        
        ctx.accounts.candidate_account.candidate_votes += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'a> {
    #[account(mut)]
    pub signer: Signer<'a>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [signer.key.as_array(), b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'a, PollAccount>,
    
    pub system_program: Program<'a, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    pub poll_id: u64,
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(256)]
    pub description: String,

    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'a> {
    #[account(mut)]
    pub signer: Signer<'a>,

    #[account(
        mut,
        seeds = [signer.key.as_array(), b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'a, PollAccount>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [signer.key.as_array(), poll_id.to_le_bytes().as_ref(), b"candidate".as_ref(), candidate_name.as_ref()],
        bump,
    )]
    pub candidate_account: Account<'a, CandidateAccount>,

    pub system_program: Program<'a, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub poll_id: u64,
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[derive(Accounts)]
pub struct Vote<'a> {
    pub poll_account: Account<'a, PollAccount>,

    #[account(mut)]
    pub candidate_account: Account<'a, CandidateAccount>,
}

#[error_code]
pub enum ProgramError {
    #[msg("Permission denied")]
    PermissionDenied,
    
    #[msg("Non voting period")]
    NonVotingPeriod,
}
