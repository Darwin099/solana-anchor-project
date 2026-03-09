use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod solana_anchor_project {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
