use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK: f64 = 25.0;

pub fn cord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    cord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = cord(x);
    let gui_y = cord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK, BLOCK],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = cord(x);
    let y = cord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK * (width as f64),
            BLOCK * (height as f64),
        ],
        con.transform,
        g,
    );
}