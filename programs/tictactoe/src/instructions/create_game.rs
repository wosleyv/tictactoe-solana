use anchor_lang::{prelude::*, solana_program::clock};

use crate::state::*;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(init, payer = player, space = 8 + std::mem::size_of::<Game>())]
    pub game: Box<Account<'info, Game>>,

    //player_x
    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;

    let clock = clock::Clock::get()?;

    let now_ts: u64 = clock.unix_timestamp.try_into().unwrap();

    // player who created the game
    let player_x = &ctx.accounts.player;

    game.authority = player_x.key();
    game.player_x = player_x.key();
    game.game_created_at_ts = now_ts;

    msg!("game initialized by player: {}", player_x.key());
    Ok(())
}
