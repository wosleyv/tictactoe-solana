use anchor_lang::prelude::*;

#[error_code]
pub enum GameErrorCode {
    #[msg("Unable to join this game")]
    UnableToJoin,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("The Game is over or hasn't started yet")]
    GameOverOrNotStarted,
    #[msg("Illegal move")]
    Illegalmove,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub enum GameState {
    WaitingPlayer,
    Started,
    PlayerXWon,
    PlayerOWon,
    Draw,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub enum Player {
    PlayerX,
    PlayerO,
}
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub enum Board {
    Empty,
    X,
    O,
}

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct Game {
    pub authority: Pubkey,

    pub state: GameState,
    pub turn: Option<Player>,

    pub player_x: Pubkey,
    pub player_o: Pubkey,

    pub game_created_at_ts: u64,

    pub board: [Board; 9],
}

impl Game {
    pub fn get_index(&self, player_move: (i32, i32)) -> i32 {
        let (x, y) = player_move;

        let index = y * 3 + x;

        return index;
    }

    pub fn is_valid_move(&self, player_move: (i32, i32)) -> bool {
        let (x, y) = player_move;

        let is_valid: bool = if x >= 0 && x <= 2 && y >= 0 && y <= 2 {
            true
        } else {
            false
        };

        return is_valid;
    }

    pub fn get_position(&self, position: (i32, i32)) -> Board {
        let (x, y) = position;

        let index = Game::get_index(self, (x, y));

        let position = self.board[index as usize];

        return position;
    }

    pub fn find_winner(&self, player_move: (i32, i32)) -> Option<GameState> {
        let (x, y) = player_move;

        let get = |x: i32, y: i32| {
            let position = Game::get_position(self, (x % 3, y % 3));

            return position;
        };

        let line = (get(x, y), get(x + 1, y), get(x + 2, y));
        let column = (get(x, y), get(x, y + 1), get(x, y + 2));

        let diagonal_left = (get(0, 0), get(1, 1), get(2, 2));
        let diagonal_right = (get(2, 0), get(1, 1), get(0, 2));

        let check = |x: (Board, Board, Board)| match x {
            (board_1, board_2, board_3) => {
                if board_1 != Board::Empty && board_1 == board_2 && board_2 == board_3 {
                    match board_1 {
                        Board::O => Some(GameState::PlayerOWon),
                        Board::X => Some(GameState::PlayerXWon),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        };

        let winner = [line, column, diagonal_left, diagonal_right]
            .into_iter()
            .find_map(|possible_wins| check(possible_wins));

        return winner;
    }

    pub fn is_first_move(&self) -> bool {
        let fist_move = self.board.iter().all(|&p| p == Board::Empty);

        return fist_move;
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::WaitingPlayer
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::Empty
    }
}
