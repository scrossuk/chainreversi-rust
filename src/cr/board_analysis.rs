// Chain Reversi Board Analysis.

use super::Board;
use super::Position;
use super::Value;

pub struct BoardAnalysis<'a> {
    board: &'a Board
}

impl BoardAnalysis<'_> {
    pub fn new(board: &Board) -> BoardAnalysis {
        return BoardAnalysis{board};
    }

    #[allow(dead_code)]
    pub fn count_all(&self, player: Value) -> u32 {
        let mut count: u32 = 0;
        for x in 0..self.board.size().width {
            for y in 0..self.board.size().height {
                if self.board.get(Position{x, y}) == player {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn count_corners(&self, player: Value) -> u32 {
        let mut count: u32 = 0;

        if self.board.get(Position{x: 0, y: 0}) == player {
            count += 1;
        }

        if self.board.get(Position{x: 0, y: self.board.size().height - 1}) == player {
            count += 1;
        }

        if self.board.get(Position{x: self.board.size().width - 1, y: 0}) == player {
            count += 1;
        }

        if self.board.get(Position{x: self.board.size().width - 1, y: self.board.size().height - 1}) == player {
            count += 1;
        }

        return count;
    }

    pub fn reachable_corners(&self) -> u32 {
        let mut count: u32 = 0;

        if self.board.is_valid(Position{x: 0, y: 0}) {
            count += 1;
        }

        if self.board.is_valid(Position{x: 0, y: self.board.size().height - 1}) {
            count += 1;
        }

        if self.board.is_valid(Position{x: self.board.size().width - 1, y: 0}) {
            count += 1;
        }

        if self.board.is_valid(Position{x: self.board.size().width - 1, y: self.board.size().height - 1}) {
            count += 1;
        }

        return count;
    }

    pub fn has_winner(&self) -> bool {
        for x in 0..self.board.size().width {
            for y in 0..self.board.size().height {
                let position = Position{x, y};
                if !self.board.is_valid(position) {
                    continue;
                }

                let mut board_copy = self.board.copy();
                board_copy.place_move(position);
                if board_copy.is_complete() {
                    return true;
                }
            }
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn get_winners(&self) -> Vec<Position> {
        let mut winners = Vec::new();

        for x in 0..self.board.size().width {
            for y in 0..self.board.size().height {
                let position = Position{x, y};
                if !self.board.is_valid(position) {
                    continue;
                }

                let mut board_copy = self.board.copy();
                board_copy.place_move(position);
                if board_copy.is_complete() {
                    winners.push(position);
                }
            }
        }

        return winners;
    }

}
