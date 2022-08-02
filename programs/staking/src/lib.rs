use anchor_lang::prelude::*;

declare_id!("DJQcSKGPXre9ZMJHGxdZhGYwKGBpEaQHPUpRzLuqhYWY");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
