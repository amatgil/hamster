mod overlay;
use std::env::args;

use hamster::{click, moveto, HWheelError};
use overlay::bring_up_overlay;

fn main() -> Result<(), HWheelError> {
    let Some(arg) = args().nth(1) else {
        return Err(HWheelError::ForgotArgument);
    };
    match &*arg {
        "overlay" => bring_up_overlay()?,
        "scrollup" => click("4")?,
        "scrolldown" => click("5")?,
        "click" => {
            let b = args().nth(2).ok_or(HWheelError::NoClickButton)?;
            // Rough check for numberiness
            let _: usize = b.parse().map_err(|_| HWheelError::ClickButtonIsntNumber)?;
            click(&b)?;
        }
        "moveto" => {
            let y = args()
                .nth(2)
                .ok_or(HWheelError::NoMoveY)?
                .parse()
                .map_err(|_| HWheelError::InvalidMoveY)?;
            let x = args()
                .nth(3)
                .ok_or(HWheelError::NoMoveX)?
                .parse()
                .map_err(|_| HWheelError::InvalidMoveX)?;

            moveto(y, x)?;
        }
        _ => {}
    }
    Ok(())
}
