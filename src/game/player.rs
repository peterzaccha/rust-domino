use macroquad::prelude::Vec2;

use super::domino::{Domino, DOMINO_WIDTH};

pub struct Player {
    pub dominos: Vec<Domino>,
    pub position: Vec2,
}

impl Player {
    pub fn stack_dominos(&mut self) {
        for (i, domino) in self.dominos.iter_mut().enumerate() {
            let x = self.position.x + (i as f32 * (DOMINO_WIDTH + 4.0));
            let y = self.position.y;
            domino.position = Vec2::new(x, y);
        }
    }
}
