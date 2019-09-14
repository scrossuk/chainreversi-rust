// Chain Reversi AI Player.

use super::Board;
use super::BoardAnalysis;
use super::Position;

use sfml::system::Clock;
use sfml::graphics::RenderWindow;

pub struct AIPlayer {
    is_waiting: bool,
    clock: Clock
}

impl AIPlayer {
    pub fn new() -> AIPlayer {
        return AIPlayer{is_waiting: false, clock: Clock::start()};
    }

    pub fn perform_move(&mut self, _window: &RenderWindow, board: &Board) -> Option<Position> {
        // This AI is very fast; too fast!
        // Hence it must wait for half a
        // second before playing each move.
        if !self.is_ready() {
            return None;
        }

        let mut best_quality: u32 = 0;
        let mut best_position = Position{x: 0, y: 0};

        for x in 0..8 {
            for y in 0..8 {
                let position = Position{x, y};
                if !board.is_valid(position) {
                    continue;
                }

                let quality = self.score_quality(board, position);
                if quality >= best_quality {
                    best_quality = quality;
                    best_position = position;
                }
            }
        }

        return Some(best_position);
    }

    fn is_ready(&mut self) -> bool {
        if !self.is_waiting {
            self.clock.restart();
            self.is_waiting = true;
            return false;
        }

        if self.clock.elapsed_time().as_seconds() < 0.5 {
            return false;
        }

        self.is_waiting = false;
        return true;
    }

    fn score_quality(&self, board: &Board, position: Position) -> u32 {
        let analysis = BoardAnalysis::new(board);

        let mut board_copy = board.copy();
        board_copy.place_move(position);

        let mut quality: u32 = 10000;

        let copy_analysis = BoardAnalysis::new(&board_copy);

        if copy_analysis.has_winner() {
            // Opponent has a winner -> this move is bad.
            return 1;
        } else if board_copy.is_complete() {
            // This move is a winner -> this move is very good.
            return 100000;
        }

        // Edges are good.
        if position.is_edge() {
            quality += 10;
        }

        // Diagonals are very good.
        if position.is_diagonal() {
            quality += 50;
        }

        // Corners are fantastic.
        quality += (copy_analysis.count_corners(board.player()) - analysis.count_corners(board.player())) * 1000;

        // Opponents getting corners is very bad.
        quality -= copy_analysis.reachable_corners() * 1000;

        return quality;
    }

    pub fn game_over(&self, _win: bool) { }
}
