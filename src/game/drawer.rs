use std::f32::consts::PI;

use macroquad::{
    prelude::{Color, Vec2, BLACK, GREEN, RED, WHITE},
    shapes::{draw_circle, draw_line, draw_rectangle, draw_rectangle_lines, draw_triangle},
    text::draw_text,
};

use super::domino::{Domino, DOMINO_HIEGHT, DOMINO_WIDTH};

trait RotateAround {
    fn rotate_around(&self, rhs: Vec2, origin: Vec2) -> Vec2;
}

impl RotateAround for Vec2 {
    fn rotate_around(&self, rhs: Vec2, origin: Vec2) -> Vec2 {
        let mut point = *self - origin;
        point = rhs.rotate(point);
        point + origin
    }
}

fn draw_rect_points(points: [Vec2; 4], color: Color) {
    draw_triangle(points[0], points[1], points[3], color);
    draw_triangle(points[1], points[2], points[3], color);
}

fn draw_points(points: usize, pos: (f32, f32), rotation: Vec2, origin: Vec2, num: &str) {
    let width = DOMINO_WIDTH;
    let height = DOMINO_HIEGHT / 2.0;
    // let center = Vec2::new(pos.0 + width / 2.0, pos.1 + height);
    // draw_rectangle(pos.0, pos.1, width, height, WHITE);
    let padding = 10.0;
    let positions = [
        Vec2::new(pos.0 + padding, pos.1 + padding),
        Vec2::new(pos.0 + padding, pos.1 + height / 2.0),
        Vec2::new(pos.0 + padding, pos.1 + height - padding),
        Vec2::new(pos.0 + width / 2.0, pos.1 + height / 2.0),
        Vec2::new(pos.0 + width - padding, pos.1 + padding),
        Vec2::new(pos.0 + width - padding, pos.1 + height / 2.0),
        Vec2::new(pos.0 + width - padding, pos.1 + height - padding),
    ];

    let x = points & 0b001;
    let y = (points & 0b010) >> 1;
    let z = (points & 0b100) >> 2;
    let output = [z, y & z, y | z, x, y | z, y & z, z];

    for i in 0..7 {
        if output[i] > 0 {
            draw_circle(
                positions[i].rotate_around(rotation, origin).x,
                positions[i].rotate_around(rotation, origin).y,
                4.0,
                BLACK,
            );
        }
    }
    let text = Vec2::new(pos.0 + width / 2.0, pos.1 + 10.0).rotate_around(rotation, origin);

    draw_text(
        format!("{}:{}", num, points).as_str(),
        text.x,
        text.y,
        8.0,
        BLACK,
    );
}
pub fn draw_domino(domino: &Domino, pos: Vec2, h: bool, hidden: bool, color: Color) {
    let mut rotation = Vec2::from_angle(0.0);
    if h {
        rotation = Vec2::from_angle(PI / 2.0);
    }

    let origin = Vec2::new(pos.x + DOMINO_WIDTH / 2.0, pos.y + DOMINO_HIEGHT / 2.0);

    let rect = [
        Vec2::new(pos.x, pos.y).rotate_around(rotation, origin),
        Vec2::new(pos.x + DOMINO_WIDTH, pos.y).rotate_around(rotation, origin),
        Vec2::new(pos.x + DOMINO_WIDTH, pos.y + DOMINO_HIEGHT).rotate_around(rotation, origin),
        Vec2::new(pos.x, pos.y + DOMINO_HIEGHT).rotate_around(rotation, origin),
    ];

    draw_rect_points(rect, color);

    if !hidden {
        let line_start =
            Vec2::new(pos.x + 5.0, pos.y + DOMINO_HIEGHT / 2.0).rotate_around(rotation, origin);
        let line_end = Vec2::new(pos.x + DOMINO_WIDTH - 5.0, pos.y + DOMINO_HIEGHT / 2.0)
            .rotate_around(rotation, origin);
        draw_line(
            line_start.x,
            line_start.y,
            line_end.x,
            line_end.y,
            2.0,
            BLACK,
        );

        draw_circle(origin.x, origin.y, 2.0, BLACK);

        let top = Vec2::new(pos.x, pos.y); //.rotate_around(rotation, origin);
        let bottom = Vec2::new(pos.x, pos.y + DOMINO_HIEGHT / 2.0); //.rotate_around(rotation, origin);

        draw_points(domino.top, (top.x, top.y), rotation, origin, "T");
        draw_points(domino.bottom, (bottom.x, bottom.y), rotation, origin, "B");
    }
}
