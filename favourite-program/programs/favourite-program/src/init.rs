use anchor_lang::prelude::*;

declare_id!("CDJZY4FxY6XFFXo4deKYXy1ELsMRwNNwt4tZHArBghuV");

#[program]
pub mod favourite_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
