mod overlay;
use std::env::args;

use hamsterwheel::*;
use overlay::bring_up_overlay;

fn main() -> Result<(), HWheelError> {
    let Some(arg) = args().nth(1) else {
        return Err(HWheelError::ForgotArgument);
    };
    match &*arg {
        "overlay" => bring_up_overlay()?,
        "scrollup" => scrollup()?,
        "scrolldown" => scrolldown()?,
        "click" => {
            let Some(b) = args().nth(2) else {
                return Err(HWheelError::NoClickButton);
            };
            let Ok(_b): Result<usize, _> = b.parse() else {
                return Err(HWheelError::ClickButtonIsntNumber);
            };
            click(&b)?
        }
        "moveto" => {
            let Some(y) = args().nth(2) else {
                return Err(HWheelError::NoMoveY);
            };
            let Ok(y) = y.parse() else {
                return Err(HWheelError::InvalidMoveY);
            };
            let Some(x) = args().nth(3) else {
                return Err(HWheelError::NoMoveX);
            };
            let Ok(x) = x.parse() else {
                return Err(HWheelError::InvalidMoveX);
            };
            moveto(y, x)?
        }
        _ => {}
    }
    Ok(())
    //bring_up_overlay();
}
