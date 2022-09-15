use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct PlayerMoves<'info> {
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

    #[account(mut)]
    pub player: Signer<'info>,
}

pub fn handler(ctx: Context<PlayerMoves>, player_move: (i32, i32)) -> Result<()> {
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

    let turn = match game.turn {
        Some(turn) => turn,
        None => return Err(error!(GameErrorCode::Unauthorized)),
    };

    // ensuring that the player is playing on their turn
    if player.key() == player_x && turn != Player::PlayerX {
        return Err(error!(GameErrorCode::Unauthorized));
    } else if player.key() == player_o && turn != Player::PlayerO {
        return Err(error!(GameErrorCode::Unauthorized));
    }

    let is_valid_move = game.is_valid_move(player_move);

    if !is_valid_move {
        return Err(error!(GameErrorCode::Illegalmove));
    }

    let position = game.get_position(player_move);

    if position != Board::Empty {
        return Err(error!(GameErrorCode::Illegalmove));
    }

    let index = game.get_index(player_move);

    if player.key() == player_x {
        game.board[index as usize] = Board::X;
        game.turn = Some(Player::PlayerO);
    } else {
        game.board[index as usize] = Board::O;
        game.turn = Some(Player::PlayerX);
    }

    let find_winner = game.find_winner(player_move);

    match find_winner {
        Some(winner) => game.state = winner,
        None => {
            let is_draw = game.board.iter().all(|&p| p != Board::Empty);
            if is_draw {
                game.state = GameState::Draw;
                game.turn = None;
            }
        }
    }

    Ok(())
}
