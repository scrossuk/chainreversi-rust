// Position on board.

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn is_edge(&self) -> bool {
        return self.x == 0 || self.y == 0 || self.x == 7 || self.y == 7;
    }

    pub fn is_diagonal(&self) -> bool {
        return self.x == self.y || (self.x + self.y) == 7;
    }
}
