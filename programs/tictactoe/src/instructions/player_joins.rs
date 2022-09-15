use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct PlayerJoins<'info> {
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    #[account(mut)]
    pub player: Signer<'info>,
}

pub fn handler(ctx: Context<PlayerJoins>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;

    if game.state != GameState::WaitingPlayer {
        return Err(error!(GameErrorCode::UnableToJoin));
    }

    if player.key() == game.player_x {
        return Err(error!(GameErrorCode::Unauthorized));
    }

    game.player_o = player.key();
    game.state = GameState::Started;
    game.turn = Some(Player::PlayerX);

    msg!("Player {} just joined", player.key());
    Ok(())
}
