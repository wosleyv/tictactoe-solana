use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Surrender<'info> {
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    #[account(mut)]
    pub player: Signer<'info>,
}

pub fn handler(ctx: Context<Surrender>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;

    let player_x = game.player_x;
    let player_o = game.player_o;

    if game.state != GameState::Started {
        return Err(error!(GameErrorCode::GameOverOrNotStarted));
    }

    if player.key() != player_x.key() && player.key() != player_o.key() {
        return Err(error!(GameErrorCode::Unauthorized));
    }

    if player.key() == player_x {
        game.state = GameState::PlayerOWon;
        game.turn = None
    } else {
        game.state = GameState::PlayerXWon;
        game.turn = None
    }

    msg!("player {} surrendered", player.key());
    Ok(())
}
