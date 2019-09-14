// Chain Reversi Human Player.

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

use super::Board;
use super::Position;

pub struct HumanPlayer { }

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        return HumanPlayer{};
    }

    pub fn perform_move(&mut self, window: &RenderWindow, board: &Board) -> Option<Position> {
        if !mouse::Button::Left.is_pressed() {
            return None;
        }

        let mouse_pixel_pos = window.mouse_position();
        let mouse_coords_pos = window.map_pixel_to_coords_current_view(&mouse_pixel_pos);
        let mouse_pos = Vector2f::new(mouse_coords_pos.x / 800.0, mouse_coords_pos.y / 800.0);

        if mouse_pos.x <= 0.0 || mouse_pos.y <= 0.0 || mouse_pos.x >= 1.0 || mouse_pos.y >= 1.0 {
            return None;
        }

        let x: usize = (mouse_pos.x * (board.size().width as f32)) as usize;
        let y: usize = (mouse_pos.y * (board.size().height as f32)) as usize;

        if x >= board.size().width || y >= board.size().height {
            // Just in case...
            return None;
        }

        let position = Position{x, y};

        if !board.is_valid(position) {
            return None;
        }

        return Some(position);
    }

    pub fn game_over(&self, _win: bool) { }
}
