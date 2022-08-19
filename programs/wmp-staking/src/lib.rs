pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wmp_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        instructions::initialize::handler(ctx)
    }

    pub fn create_stake_pool(ctx: Context<CreateStakePool>) -> ProgramResult {
        instructions::create_stake_pool::handler(ctx)
    }

    pub fn create_stake_entry(ctx: Context<CreateStakeEntry>) -> ProgramResult {
        instructions::create_stake_entry::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>) -> ProgramResult {
        instructions::stake::handler(ctx)
    }

    pub fn unstake(ctx: Context<Unstake>) -> ProgramResult {
        instructions::unstake::handler(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        instructions::claim_rewards::handler(ctx)
    }
}
