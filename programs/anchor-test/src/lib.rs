use anchor_lang::prelude::*;

declare_id!("84n3QwFJaRe197URt8cEVceALK55q6T21TZViCJ8PaJe");

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
