// Chain Reversi Board.

use super::Position;
use super::Value;

#[derive(Clone, Copy, PartialEq)]
pub struct BoardSize {
    pub width: usize,
    pub height: usize
}

pub struct Board {
    data: Vec<Value>,
    current_player: Value,
    size: BoardSize
}

impl Board {
    pub fn new(size: BoardSize, initial_player: Value) -> Board {
        let mut data = Vec::new();

        for _p in 0..size.width * size.height {
            data.push(Value::Empty);
        }

        return Board{data, current_player: initial_player, size};
    }

    pub fn copy(&self) -> Board {
        let mut data = Vec::new();

        for p in 0..self.size.width * self.size.height {
            data.push(self.data[p]);
        }

        return Board{data, current_player: self.current_player, size: self.size};
    }

    pub fn is_valid(&self, position: Position) -> bool {
        if self.get(position) != Value::Empty {
            return false;
        }

        for xi in -1..2 {
            for yi in -1..2 {
                if xi == 0 && yi == 0 {
                    continue;
                }

                if is_valid_for_direction(self, position.x, position.y, xi, yi) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn place_move(&mut self, position: Position) {
        self.set(position, self.player());

        self.update();

        self.current_player = self.opponent();
    }

    pub fn player(&self) -> Value {
        return self.current_player;
    }

    pub fn opponent(&self) -> Value {
        return self.current_player.opposite();
    }

    pub fn size(&self) -> BoardSize {
        return self.size;
    }

    pub fn is_complete(&self) -> bool {
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                if self.is_valid(Position{x, y}) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn get(&self, position: Position) -> Value {
        return self.data[position.x + position.y * self.size.width];
    }

    pub fn set(&mut self, position: Position, value: Value) {
        self.data[position.x + position.y * self.size.width] = value;
    }

    pub fn update(&mut self) {
        let mut has_changed = true;

        while has_changed {
            has_changed = false;

            for x in 0..self.size.width {
                for y in 0..self.size.height {
                    if self.get(Position{x, y}) != self.player() {
                        continue;
                    }

                    for xi in -1..2 {
                        for yi in -1..2 {
                            if xi == 0 && yi == 0 {
                                continue;
                            }

                            if update_for_direction(self, x, y, xi, yi) {
                                has_changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

}

fn is_valid_for_direction(board: &Board, x: usize, y: usize, xi: isize, yi: isize) -> bool {
    let mut flag_opponent = false;

    let mut cx = x as isize + xi;
    let mut cy = y as isize + yi;

    while cx >= 0 && cx < board.size().width as isize && cy >= 0 && cy < board.size().height as isize {
        let position = Position{x: cx as usize, y: cy as usize};

        if board.get(position) == board.opponent() {
            flag_opponent = true;
            cx += xi;
            cy += yi;
        } else {
            if flag_opponent && board.get(position) == board.player() {
                return true;
            } else {
                break;
            }
        }
    }

    return false;
}

fn update_for_direction(board: &mut Board, x: usize, y: usize, xi: isize, yi: isize) -> bool {
    if !is_valid_for_direction(board, x, y, xi, yi) {
        return false;
    }

    let mut cx: isize = x as isize + xi;
    let mut cy: isize = y as isize + yi;

    while cx >= 0 && cx < board.size().width as isize && cy >= 0 && cy < board.size().height as isize {
        let position = Position{x: cx as usize, y: cy as usize};

        if board.get(position) == board.opponent() {
            board.set(position, board.player());
            cx += xi;
            cy += yi;
        } else {
            break;
        }
    }

    return true;
}
