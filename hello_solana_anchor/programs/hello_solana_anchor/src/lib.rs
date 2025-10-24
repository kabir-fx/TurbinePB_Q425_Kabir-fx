use anchor_lang::prelude::*;

declare_id!("DyjRE2i1kWEJ3veCp4ohNjLHmy8LGYQZvMM1fvvvjsKg");

#[program]
pub mod hello_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Helloo Solana!!! from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
