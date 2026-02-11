use anchor_lang::prelude::*;

declare_id!("3BxPymFpACUfcSpNrW813gy9EXQMZ99GA9sGCoehxZ8m");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
