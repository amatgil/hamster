use raylib::prelude::*;

pub const HAMSTER_OPACITY: f32 = 0.2;
pub const HAMSTER_BACKGROUND: Color = Color {
    // TODO: Make this look good
    r: 62,
    g: 176,
    b: 60,
    a: 0,
};

/// Elements per row
pub const GRID_HEIGHT: i32 = 13;
/// Elements per column
pub const GRID_WIDTH: i32 = 13;

pub const CHILD_GAP: f32 = 2.5;

pub const PADDING_W: i32 = 0;
pub const PADDING_H: i32 = 0;

pub const FONT_SIZE: i32 = 100;
pub const TEXT_COLOR: Color = Color::BLACK;

pub const TARGET_FPS: u32 = 20;
