use anchor_lang::prelude::*;

declare_id!("AkzYusxQZfidYsboUbjgh24nuLzNXPtsYS98EnfFeYbE");

#[program]
pub mod pamol_favorite_prj {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
