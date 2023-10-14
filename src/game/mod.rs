use std::collections::VecDeque;

use macroquad::{
    prelude::{Vec2, BLACK, GREEN, WHITE},
    shapes::{draw_rectangle, draw_rectangle_lines},
    window::{screen_height, screen_width},
};

use self::{
    domino::{Domino, DominoLocator, DominoPlace, DOMINO_HIEGHT, DOMINO_WIDTH},
    player::Player,
};

pub mod domino;
mod drawer;
pub mod player;
pub mod player_move;

#[derive(Debug)]
pub enum GameMode {
    SelectDominoToPlay,
    SelectGround,
    PullFromTheSide,
}

pub struct Game {
    players: [Player; 2],
    ground: VecDeque<Domino>,
    side_dominos: Vec<Domino>,
    turn: bool,
    mode: GameMode,
    selected_domino_idx: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        let all = Domino::new_all();

        Self {
            players: [
                Player {
                    dominos: all[0..6]
                        .to_vec()
                        .iter()
                        .enumerate()
                        .map(|(i, d)| {
                            let mut new_d = d.clone();
                            let x = 20.0 + (i as f32 * (DOMINO_WIDTH + 4.0));
                            let y = 20.0;
                            new_d.position = Vec2::new(x, y);

                            new_d
                        })
                        .collect(),
                    position: Vec2::new(20.0, 20.0),
                },
                Player {
                    dominos: all[6..13]
                        .to_vec()
                        .iter()
                        .enumerate()
                        .map(|(i, d)| {
                            let mut new_d = d.clone();
                            let x = 20.0 + (i as f32 * (DOMINO_WIDTH + 4.0));
                            let y = screen_height() - 120.0;
                            new_d.position = Vec2::new(x, y);
                            new_d
                        })
                        .collect(),
                    position: Vec2::new(20.0, screen_height() - 120.0),
                },
            ],
            ground: VecDeque::from(
                all[13..14]
                    .to_vec()
                    .iter()
                    .enumerate()
                    .map(|(i, d)| {
                        let mut new_d = d.clone();
                        let x = 200.0 + (i as f32 * (DOMINO_HIEGHT + 4.0));
                        let y = screen_height() / 4.0;
                        new_d.position = Vec2::new(x, y);
                        new_d.h = true;
                        new_d
                    })
                    .collect::<Vec<Domino>>(),
            ),
            side_dominos: all[14..]
                .to_vec()
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let mut new_d = d.clone();
                    let x = screen_width() - 100.0;
                    let y = i as f32 * (DOMINO_WIDTH + 4.0);
                    new_d.position = Vec2::new(x, y);
                    new_d.h = true;
                    new_d.hidden = true;
                    new_d
                })
                .collect(),
            turn: true,
            mode: GameMode::SelectDominoToPlay,
            selected_domino_idx: None,
        }
    }

    fn valid_play(&self, domino: &Domino) -> bool {
        if self.ground.len() == 0 {
            return true;
        }

        let ground_top = self.ground.back().unwrap();
        let ground_bottom = self.ground.front().unwrap();

        return *domino == ground_top.top || *domino == ground_bottom.bottom;
    }

    pub fn get_domino_at_position(&self, position: (f32, f32)) -> Option<DominoLocator> {
        let valid = self.players[self.turn as usize]
            .dominos
            .iter()
            .position(|domino| {
                domino.is_position_inside(Vec2::from(position)) && self.valid_play(domino)
            })?;
        return Some(DominoLocator::new(valid, DominoPlace::Player(self.turn)));
    }

    pub fn click(&mut self, position: (f32, f32)) -> () {
        println!("{:?}", &self.mode);
        match self.mode {
            GameMode::SelectDominoToPlay => {
                let valid = self.get_domino_at_position(position);
                if let Some(locator) = valid {
                    // let mut domino = self.players[self.turn as usize].dominos.remove(locator.idx);
                    // domino.put_after(self.ground.back().unwrap());
                    // self.ground.push_back(domino);
                    // self.turn = !self.turn;
                    self.selected_domino_idx = Some(locator.idx);
                    self.mode = GameMode::SelectGround
                }
            }
            GameMode::SelectGround => {
                let player = &mut self.players[self.turn as usize];
                if self
                    .ground
                    .back()
                    .unwrap()
                    .is_position_inside(Vec2::from(position))
                {
                    let mut domino = player
                        .dominos
                        .remove(self.selected_domino_idx.take().unwrap());
                    domino.put_after(self.ground.back().unwrap());
                    self.ground.push_back(domino);
                    self.turn = !self.turn;
                    self.mode = GameMode::SelectDominoToPlay;
                    player.stack_dominos();
                } else if self
                    .ground
                    .front()
                    .unwrap()
                    .is_position_inside(Vec2::from(position))
                {
                    let mut domino = player
                        .dominos
                        .remove(self.selected_domino_idx.take().unwrap());
                    domino.put_before(self.ground.front().unwrap());
                    self.ground.push_front(domino);
                    self.turn = !self.turn;
                    self.mode = GameMode::SelectDominoToPlay;
                    player.stack_dominos();
                }
            }
            GameMode::PullFromTheSide => todo!(),
        }
    }

    pub fn render(&self) -> () {
        for domino in &self.side_dominos {
            domino.render(WHITE);
        }

        for domino in &self.ground {
            domino.render(WHITE);
        }

        for (i, player) in self.players.iter().enumerate() {
            for domino in &player.dominos {
                if self.turn as usize == i && self.valid_play(domino) {
                    domino.render(GREEN);
                } else {
                    domino.render(WHITE);
                }
            }
        }
    }
}
