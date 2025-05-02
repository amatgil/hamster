use std::{
    error::Error,
    fmt::Display,
    process::{Command, ExitCode, Termination},
};

mod keys;
pub use keys::*;
use raylib::prelude::*;

pub const HAMSTER_OPACITY: f32 = 0.40;
pub const HAMSTER_BACKGROUND: Color = Color {
    // TODO: Make this look good
    r: 250,
    g: 179,
    b: 135,
    a: 0,
};

pub const GRID_HEIGHT: i32 = 13;
pub const GRID_WIDTH: i32 = 10;

pub const CHILD_GAP: f32 = 2.5; // Line thickness

pub const PADDING_W: i32 = 0;
pub const PADDING_H: i32 = 0;

pub const FONT_SIZE: i32 = 100;
pub const TEXT_COLOR: Color = Color::BLACK;

pub const TARGET_FPS: u32 = 20;

pub const LOCKED_RECT_COLOR: Color = Color::BLUE;
pub const LOCKED_RECT_SIDE: i32 = 30;

pub const OVERLAY_CLOSE_CLICK_DELAY_MS: u64 = 200;

#[derive(Debug)]
pub enum HWheelError {
    ForgotArgument,
    UnrecognizedArg(String),
    NoClickButton,
    ClickButtonIsntNumber,
    ClickButtonIsNotButton,
    CouldNotReachXDoTool(Box<dyn Error>),
    NoMoveY,
    NoMoveX,
    InvalidMoveY,
    InvalidMoveX,
    MonitorIsntInteger,
}

impl Display for HWheelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ForgotArgument => "no argument given",
            Self::UnrecognizedArg(s) => &format!("could not recognize argument: {s}"),
            Self::NoClickButton => "click command sent with no button number",
            Self::ClickButtonIsntNumber => "button number must be a number",
            Self::ClickButtonIsNotButton => "button number was not 1, 2 or 3",
            Self::CouldNotReachXDoTool(e) => {
                &format!("could not run 'xdotool', is it installed?: {e}")
            }
            Self::NoMoveY => "moved without providing row",
            Self::NoMoveX => "moved without providing column",
            Self::InvalidMoveY => "invalid move row",
            Self::InvalidMoveX => "invalid move column",
            Self::MonitorIsntInteger => "monitor is not integer",
        };

        write!(f, "{s}")
    }
}
impl Termination for HWheelError {
    fn report(self) -> std::process::ExitCode {
        match self {
            Self::ForgotArgument => ExitCode::from(10),
            Self::UnrecognizedArg(_) => ExitCode::from(11),
            Self::NoClickButton => ExitCode::from(12),
            Self::ClickButtonIsntNumber => ExitCode::from(13),
            Self::ClickButtonIsNotButton => ExitCode::from(14),
            Self::CouldNotReachXDoTool(_e) => ExitCode::from(15),
            Self::NoMoveY => ExitCode::from(16),
            Self::NoMoveX => ExitCode::from(17),
            Self::InvalidMoveY => ExitCode::from(18),
            Self::InvalidMoveX => ExitCode::from(19),
            Self::MonitorIsntInteger => ExitCode::from(20),
        }
    }
}

pub fn click(btn: &str) -> Result<(), HWheelError> {
    match Command::new("xdotool").arg("click").arg(btn).spawn() {
        Ok(_c) => Ok(()),
        Err(e) => Err(HWheelError::CouldNotReachXDoTool(Box::new(e))),
    }
}

pub fn moveto(y: usize, x: usize) -> Result<(), HWheelError> {
    let mut c = Command::new("xdotool"); // sigh, forced bindings

    let c = c
        .arg("mousemove")
        .arg("--sync")
        .arg(x.to_string())
        .arg(y.to_string());

    match c.spawn() {
        Ok(_c) => Ok(()),
        Err(e) => Err(HWheelError::CouldNotReachXDoTool(Box::new(e))),
    }
}

pub fn moveto_relative(dy: isize, dx: isize) -> Result<(), HWheelError> {
    let mut c = Command::new("xdotool"); // sigh, forced bindings

    let c = c
        .arg("mousemove_relative")
        .arg("--sync")
        .arg("--")
        .arg(dx.to_string())
        .arg(dy.to_string());

    match c.spawn() {
        Ok(_c) => Ok(()),
        Err(e) => Err(HWheelError::CouldNotReachXDoTool(Box::new(e))),
    }
}
