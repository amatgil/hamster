mod overlay;
use std::env::args;

use hamsterwheel::*;
use overlay::bring_up_overlay;

fn main() -> Result<(), HWheelError> {
    let Some(arg) = args().nth(1) else {
        return Err(HWheelError::ForgotArgument);
    };
    match &*arg {
        "overlay" => bring_up_overlay(),
        "scrollup" => scrollup(),
        "scrolldown" => scrolldown(),
        "click" => {
            let Some(b) = args().nth(2) else {
                return Err(HWheelError::NoClickButton);
            };
            let Ok(b) = b.parse() else {
                return Err(HWheelError::ClickButtonIsntNumber);
            };
            click(b)?;
        }
        _ => {}
    }
    Ok(())
    //bring_up_overlay();
}
