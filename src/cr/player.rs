// Chain Reversi Player API.

use super::AIPlayer;
use super::Board;
use super::HumanPlayer;
use super::Position;

use sfml::graphics::RenderWindow;

pub enum Player {
    AI(AIPlayer),
    Human(HumanPlayer)
}

impl Player {
    pub fn perform_move(&mut self, window: &RenderWindow, board: &Board) -> Option<Position> {
        match self {
            Player::AI(player) => player.perform_move(window, board),
            Player::Human(player) => player.perform_move(window, board)
        }
    }

    pub fn game_over(&self, win: bool) {
        match &self {
            Player::AI(player) => player.game_over(win),
            Player::Human(player) => player.game_over(win)
        }
    }
}
