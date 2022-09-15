use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod instructions;
pub mod state;

#[program]
pub mod tictactoe {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        instructions::create_game::handler(ctx)
    }

    pub fn player_joins(ctx: Context<PlayerJoins>) -> Result<()> {
        instructions::player_joins::handler(ctx)
    }

    pub fn player_moves(ctx: Context<PlayerMoves>, x: i32, y: i32) -> Result<()> {
        instructions::player_moves::handler(ctx, (x, y))
    }

    pub fn surrender(ctx: Context<Surrender>) -> Result<()> {
        instructions::surrender::handler(ctx)
    }
}
