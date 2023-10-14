use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::{draw_rectangle, Color, Vec2, BLACK, GREEN, WHITE};
use macroquad::rand::{srand, ChooseRandom};
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle_lines};
use macroquad::text::draw_text;

use super::drawer::draw_domino;

pub const DOMINO_WIDTH: f32 = 50.0;
pub const DOMINO_HIEGHT: f32 = 100.0;
#[derive(Clone, Debug)]
pub struct Domino {
    pub top: usize,
    pub bottom: usize,
    pub position: Vec2,
    pub h: bool,
    pub hidden: bool,
}

pub enum DominoPlace {
    Player(bool),
    Ground,
    Side,
}

pub struct DominoLocator {
    pub idx: usize,
    pub place: DominoPlace,
}

impl DominoLocator {
    pub fn new(idx: usize, place: DominoPlace) -> Self {
        Self { idx, place }
    }
}

impl PartialEq<usize> for Domino {
    fn eq(&self, other: &usize) -> bool {
        self.top == *other || self.bottom == *other
    }
}

impl Domino {
    pub fn new(top: usize, bottom: usize, position: Vec2) -> Self {
        Self {
            top,
            bottom,
            position,
            h: false,
            hidden: false,
        }
    }
    pub fn new_all() -> Vec<Self> {
        let mut all: Vec<Self> = vec![];
        for i in 0..7 {
            for j in 0..i + 1 {
                all.push(Self::new(i, j, Vec2::new(0.0, 0.0)))
            }
        }
        srand(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
        );
        all.shuffle();
        return all;
    }

    fn reverse(&mut self) {
        let temp = self.top;
        self.top = self.bottom;
        self.bottom = temp
    }

    pub fn put_after(&mut self, other: &Domino) {
        if self.bottom != other.top && self.top != other.top {
            panic!("Cannot place diffrent dominos after each others {other:?} -> {self:?}")
        }

        if self.top == other.top {
            self.reverse()
        }

        self.h = other.h;
        self.position.x = other.position.x + DOMINO_HIEGHT + 4.0;
        self.position.y = other.position.y;
    }
    pub fn put_before(&mut self, other: &Domino) {
        if self.bottom != other.bottom && self.top != other.bottom {
            panic!("Cannot place diffrent dominos after each others {other:?} -> {self:?}")
        }

        if self.bottom == other.bottom {
            self.reverse()
        }

        if other.h {
            self.h = other.h;
            if (other.position.x - DOMINO_HIEGHT - 4.0) < 0.0 {
                self.h = false;
                self.position.x = other.position.x - DOMINO_HIEGHT + DOMINO_WIDTH / 2.0 - 4.0;
                self.position.y = other.position.y + DOMINO_WIDTH / 2.0;
            } else {
                self.position.x = other.position.x - DOMINO_HIEGHT - 4.0;
                self.position.y = other.position.y;
            }
        } else {
            self.position.y = other.position.y + DOMINO_HIEGHT + 4.0;
            self.position.x = other.position.x;
        }
    }

    pub fn is_position_inside(&self, position: Vec2) -> bool {
        if self.h {
            return position.x > self.position.x
                && position.x < self.position.x + DOMINO_HIEGHT
                && position.y > self.position.y
                && position.y < self.position.y + DOMINO_WIDTH;
        }
        return position.x > self.position.x
            && position.x < self.position.x + DOMINO_WIDTH
            && position.y > self.position.y
            && position.y < self.position.y + DOMINO_HIEGHT;
    }

    pub fn render(&self, color: Color) -> () {
        draw_domino(self, self.position, self.h, self.hidden, color)
    }
}
