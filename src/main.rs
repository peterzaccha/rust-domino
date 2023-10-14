mod game;

use game::{player_move::PlayerMove, player_move::PlayerMoveKind, Game};
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();
    let mut player_move: Option<PlayerMove> = None;
    loop {
        clear_background(GRAY);
        let mouse_pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{mouse_pos:?}");
            game.click(mouse_pos);
            let locator = game.get_domino_at_position(mouse_pos);

            if let Some(locator) = locator {
                player_move = Some(PlayerMove::new(locator.idx, PlayerMoveKind::PutToBack));
            }
        }
        game.render();
        next_frame().await
    }
}
