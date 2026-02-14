use anchor_lang::prelude::*;

declare_id!("3BxPymFpACUfcSpNrW813gy9EXQMZ99GA9sGCoehxZ8m");

#[error_code]
pub enum CounterError {
    #[msg("Counter overflow: addition would exceed u64::MAX")]
    MathOverflow,
    #[msg("Counter underflow: subtraction would go below zero")]
    MathUnderflow,
}

/// Seeds for PDA derivation - centralized for consistency and collision prevention
pub mod seeds {
    pub const GLOBAL_COUNTER: &[u8] = b"global_counter";
    pub const USER_COUNTER: &[u8] = b"user_counter";
}

#[program]
pub mod anchor_test {
    use super::*;

    /// Initialize the global counter. Callable once per program deployment.
    pub fn initialize_global_counter(ctx: Context<InitializeGlobalCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.global_counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = 0;
        counter.bump = ctx.bumps.global_counter;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(GlobalCounterEvent {
            event_type: CounterEventType::Initialized,
            authority: counter.authority,
            count: 0,
            timestamp: counter.updated_at,
        });

        msg!("Global counter initialized by: {:?}", counter.authority);
        Ok(())
    }

    /// Initialize a user-specific counter. Each user can have one counter.
    pub fn initialize_user_counter(ctx: Context<InitializeUserCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.user_counter;
        counter.owner = ctx.accounts.owner.key();
        counter.count = 0;
        counter.bump = ctx.bumps.user_counter;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(UserCounterEvent {
            event_type: CounterEventType::Initialized,
            owner: counter.owner,
            count: 0,
            timestamp: counter.updated_at,
        });

        msg!("User counter initialized for: {:?}", counter.owner);
        Ok(())
    }

    /// Increment global counter. Only authority can call.
    pub fn increment_global(ctx: Context<IncrementGlobal>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.global_counter;
        counter.count = counter
            .count
            .checked_add(amount)
            .ok_or(CounterError::MathOverflow)?;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(GlobalCounterEvent {
            event_type: CounterEventType::Incremented,
            authority: counter.authority,
            count: counter.count,
            timestamp: counter.updated_at,
        });

        msg!("Global counter incremented by {} to {}", amount, counter.count);
        Ok(())
    }

    /// Decrement global counter. Only authority can call. Prevents underflow.
    pub fn decrement_global(ctx: Context<DecrementGlobal>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.global_counter;
        counter.count = counter
            .count
            .checked_sub(amount)
            .ok_or(CounterError::MathUnderflow)?;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(GlobalCounterEvent {
            event_type: CounterEventType::Decremented,
            authority: counter.authority,
            count: counter.count,
            timestamp: counter.updated_at,
        });

        msg!("Global counter decremented by {} to {}", amount, counter.count);
        Ok(())
    }

    /// Increment user counter. Only owner can call.
    pub fn increment_user(ctx: Context<IncrementUser>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.user_counter;
        counter.count = counter
            .count
            .checked_add(amount)
            .ok_or(CounterError::MathOverflow)?;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(UserCounterEvent {
            event_type: CounterEventType::Incremented,
            owner: counter.owner,
            count: counter.count,
            timestamp: counter.updated_at,
        });

        msg!("User counter incremented by {} to {}", amount, counter.count);
        Ok(())
    }

    /// Decrement user counter. Only owner can call. Prevents underflow.
    pub fn decrement_user(ctx: Context<DecrementUser>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.user_counter;
        counter.count = counter
            .count
            .checked_sub(amount)
            .ok_or(CounterError::MathUnderflow)?;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(UserCounterEvent {
            event_type: CounterEventType::Decremented,
            owner: counter.owner,
            count: counter.count,
            timestamp: counter.updated_at,
        });

        msg!("User counter decremented by {} to {}", amount, counter.count);
        Ok(())
    }

    /// Transfer global counter authority to a new address. Only current authority can call.
    pub fn transfer_global_authority(ctx: Context<TransferGlobalAuthority>, new_authority: Pubkey) -> Result<()> {
        let counter = &mut ctx.accounts.global_counter;
        let old_authority = counter.authority;
        counter.authority = new_authority;
        counter.updated_at = Clock::get()?.unix_timestamp;

        emit!(GlobalCounterEvent {
            event_type: CounterEventType::Initialized, // reuse for authority change
            authority: counter.authority,
            count: counter.count,
            timestamp: counter.updated_at,
        });

        msg!("Global counter authority transferred from {:?} to {:?}", old_authority, new_authority);
        Ok(())
    }

    /// Close user counter account and reclaim rent to owner. Only owner can call.
    pub fn close_user_counter(ctx: Context<CloseUserCounter>) -> Result<()> {
        msg!("User counter closed for {:?}", ctx.accounts.user_counter.owner);
        Ok(())
    }

    /// Legacy greeting - kept for backward compatibility
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

// ============ Account Structures ============

/// Global counter state - single PDA per program
#[account]
#[derive(Default)]
pub struct GlobalCounter {
    /// Authority that can increment/decrement
    pub authority: Pubkey,
    /// Current count value
    pub count: u64,
    /// PDA bump - stored for client convenience and CPI
    pub bump: u8,
    /// Last update timestamp
    pub updated_at: i64,
}

impl GlobalCounter {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8;
}

/// Per-user counter state - one PDA per user
#[account]
#[derive(Default)]
pub struct UserCounter {
    /// Owner of this counter
    pub owner: Pubkey,
    /// Current count value
    pub count: u64,
    /// PDA bump
    pub bump: u8,
    /// Last update timestamp
    pub updated_at: i64,
}

impl UserCounter {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8;
}

// ============ Events ============

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum CounterEventType {
    Initialized,
    Incremented,
    Decremented,
}

#[event]
pub struct GlobalCounterEvent {
    pub event_type: CounterEventType,
    pub authority: Pubkey,
    pub count: u64,
    pub timestamp: i64,
}

#[event]
pub struct UserCounterEvent {
    pub event_type: CounterEventType,
    pub owner: Pubkey,
    pub count: u64,
    pub timestamp: i64,
}

// ============ Context Structs ============

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct InitializeGlobalCounter<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GlobalCounter::LEN,
        seeds = [seeds::GLOBAL_COUNTER],
        bump
    )]
    pub global_counter: Account<'info, GlobalCounter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserCounter<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + UserCounter::LEN,
        seeds = [seeds::USER_COUNTER, owner.key().as_ref()],
        bump
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementGlobal<'info> {
    #[account(
        mut,
        seeds = [seeds::GLOBAL_COUNTER],
        bump = global_counter.bump,
        has_one = authority
    )]
    pub global_counter: Account<'info, GlobalCounter>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementGlobal<'info> {
    #[account(
        mut,
        seeds = [seeds::GLOBAL_COUNTER],
        bump = global_counter.bump,
        has_one = authority
    )]
    pub global_counter: Account<'info, GlobalCounter>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementUser<'info> {
    #[account(
        mut,
        seeds = [seeds::USER_COUNTER, owner.key().as_ref()],
        bump = user_counter.bump,
        has_one = owner
    )]
    pub user_counter: Account<'info, UserCounter>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementUser<'info> {
    #[account(
        mut,
        seeds = [seeds::USER_COUNTER, owner.key().as_ref()],
        bump = user_counter.bump,
        has_one = owner
    )]
    pub user_counter: Account<'info, UserCounter>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferGlobalAuthority<'info> {
    #[account(
        mut,
        seeds = [seeds::GLOBAL_COUNTER],
        bump = global_counter.bump,
        has_one = authority
    )]
    pub global_counter: Account<'info, GlobalCounter>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseUserCounter<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [seeds::USER_COUNTER, owner.key().as_ref()],
        bump = user_counter.bump,
        has_one = owner
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(mut)]
    pub owner: Signer<'info>,
}
