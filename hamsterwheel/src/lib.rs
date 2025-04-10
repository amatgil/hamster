use std::{
    error::Error,
    fmt::Display,
    process::{ExitCode, Termination},
};

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

#[derive(Debug)]
pub enum HWheelReturn {
    Success,
    ForgotArgument,
    UnrecognizedArg(String),
    NoClickButton,
    ClickButtonIsntNumber,
}

impl Display for HWheelReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Success => "finished with no problems",
            Self::ForgotArgument => "no argument given",
            Self::UnrecognizedArg(s) => &format!("could not recognize argument: {s}"),
            Self::NoClickButton => "click command sent with no button number",
            Self::ClickButtonIsntNumber => "button number must be a number",
        };

        write!(f, "{}", s)
    }
}
impl Termination for HWheelReturn {
    fn report(self) -> std::process::ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::ForgotArgument => ExitCode::from(10),
            Self::UnrecognizedArg(_) => ExitCode::from(11),
            Self::NoClickButton => ExitCode::from(12),
            Self::ClickButtonIsntNumber => ExitCode::from(13),
        }
    }
}

pub fn scrollup() {
    todo!()
}
pub fn scrolldown() {
    todo!()
}
pub fn click(button: usize) {
    if button < 1 || button > 5 {
        //bail("")
    }
    todo!()
}
