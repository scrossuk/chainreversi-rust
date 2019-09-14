extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

mod cr;

fn initialize_board(board: &mut cr::Board) {
    for x in 0..board.size().width {
        for y in 0..board.size().height {
            board.set(cr::Position{x, y}, cr::Value::Empty);
        }
    }

    let position = rand::random();

    let width = board.size().width;
    let height = board.size().height;

    // Add initial positions.
    board.set(cr::Position{x: width / 2 - 1, y: height / 2 - 1}, if position { cr::Value::Red } else { cr::Value::Blue });
    board.set(cr::Position{x: width / 2, y: height / 2},         if position { cr::Value::Red } else { cr::Value::Blue });
    board.set(cr::Position{x: width / 2 - 1, y: height / 2},     if position { cr::Value::Blue } else { cr::Value::Red });
    board.set(cr::Position{x: width / 2, y: height / 2 - 1},     if position { cr::Value::Blue } else { cr::Value::Red });
}

fn play_game(window: &mut RenderWindow, player0: &mut cr::Player, player1: &mut cr::Player) {
    let board_size = cr::BoardSize{width: 8, height: 8};
    let initial_colour = if rand::random() { cr::Value::Red } else { cr::Value::Blue };
    let mut board = cr::Board::new(board_size, initial_colour);

    let mut last_winner = cr::Value::Red;

    initialize_board(&mut board);

    // Whether a game is currently active.
    let mut playing = true;

    // Whether p0 (false) or p1 (true) is the current player.
    let mut current_player: bool = rand::random();

    let mut clock = Clock::start();

    let font = Font::from_file("data/font.ttf").unwrap();

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                },
                _ => {},
            }
        }

        if playing {
            if !board.is_complete() {
                if !current_player {
                    let optional_position = player0.perform_move(&window, &board);

                    match optional_position {
                        Some(position) => {
                            board.place_move(position);
                            current_player = true;
                        },
                        None => {}
                    }
                } else {
                    let optional_position = player1.perform_move(&window, &board);

                    match optional_position {
                        Some(position) => {
                            board.place_move(position);
                            current_player = false;
                        },
                        None => {}
                    }
                }
            } else {
                // Indicate the result to players.
                if !current_player {
                    player0.game_over(false);
                    player1.game_over(true);
                } else {
                    player0.game_over(true);
                    player1.game_over(false);
                }

                playing = false;
                last_winner = board.opponent();
            }
            clock.restart();
        } else if clock.elapsed_time().as_seconds() > 5.0 {
            playing = true;
            current_player = rand::random();
            initialize_board(&mut board);
        }

        window.clear(&Color::BLACK);

        for x in 0..board.size().width {
            for y in 0..board.size().height {
                let mut background_circle = CircleShape::default();
                background_circle.set_radius(45.0);
                background_circle.set_fill_color(&Color::rgb(20, 20, 20));
                background_circle.set_position(Vector2f::new(x as f32 * 100.0 + 5.0, y as f32 * 100.0 + 5.0));
                window.draw(&background_circle);

                let position = cr::Position{x, y};

                let mut foreground_circle = CircleShape::default();
                foreground_circle.set_radius(40.0);

                match board.get(position) {
                    cr::Value::Red => {
                        if playing {
                            foreground_circle.set_fill_color(&Color::rgb(255, 0, 0));
                        } else {
                            foreground_circle.set_fill_color(&Color::rgb(150, 0, 0));
                        }
                    }
                    cr::Value::Blue => {
                        if playing {
                            foreground_circle.set_fill_color(&Color::rgb(0, 0, 255));
                        } else {
                            foreground_circle.set_fill_color(&Color::rgb(0, 0, 150));
                        }
                    }
                    cr::Value::Empty => {
                        if board.is_valid(position) {
                            foreground_circle.set_fill_color(&Color::rgb(70, 70, 70));
                        } else {
                            foreground_circle.set_fill_color(&Color::rgb(30, 30, 30));
                        }
                    }
                }

                foreground_circle.set_position(Vector2f::new(x as f32 * 100.0 + 10.0, y as f32 * 100.0 + 10.0));
                window.draw(&foreground_circle);
            }
        }

        if !playing {
            let mut winner_text = Text::default();
            winner_text.set_font(&font);
            winner_text.set_character_size(100);
            winner_text.set_fill_color(&Color::WHITE);

            match last_winner {
                cr::Value::Red => winner_text.set_string("Red wins!"),
                cr::Value::Blue => winner_text.set_string("Blue wins!"),
                cr::Value::Empty => {}
            }

            let text_local_rect = winner_text.local_bounds();
            winner_text.set_origin(Vector2f::new(text_local_rect.left + text_local_rect.width / 2.0,
                    text_local_rect.top + text_local_rect.height / 2.0));

            winner_text.set_position(Vector2f::new(400.0, 400.0));

            let text_global_rect = winner_text.global_bounds();

            let extra_width = text_global_rect.width * 0.5;
            let extra_height = text_global_rect.height * 0.5;

            let mut winner_text_background = RectangleShape::new();
            winner_text_background.set_size(Vector2f::new(text_global_rect.width + extra_width, text_global_rect.height + extra_height));
            winner_text_background.set_position(Vector2f::new(text_global_rect.left - extra_width / 2.0,
                text_global_rect.top - extra_height / 2.0));
            winner_text_background.set_fill_color(&Color::rgba(0, 0, 0, 200));
            window.draw(&winner_text_background);

            window.draw(&winner_text);
        }

        window.display();
    }
}

fn create_player(player_string: &str) -> Option<cr::Player> {
    if player_string == "ai" {
        return Some(cr::Player::AI(cr::AIPlayer::new()));
    } else if player_string == "human" {
        return Some(cr::Player::Human(cr::HumanPlayer::new()));
    } else {
        return None;
    }
}

fn main() {
    let usage = "chainReversi [human|ai] [human|ai] (e.g. chainReversi human ai)";

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Incorrect number of arguments! You need to give 2 arguments:");
        println!("    {}", usage);
        return;
    }

    let mut window = RenderWindow::new(
        (800, 800),
        "Chain Reversi",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(25);

    let mut view = View::from_rect(&FloatRect::new(0.0, 0.0, 800.0, 800.0));
    window.set_view(&mut view);

    let mut player0 = match create_player(&args[1]) {
        Some(r) => r,
        None => {
            println!("ERROR: Invalid player name {}.", &args[1]);
            return;
        }
    };
    let mut player1 = match create_player(&args[2]) {
        Some(r) => r,
        None => {
            println!("ERROR: Invalid player name {}.", &args[2]);
            return;
        }
    };

    play_game(&mut window, &mut player0, &mut player1);
}

