mod overlay;
use std::env::args;

use hamsterwheel::*;
use overlay::bring_up_overlay;

fn main() -> HWheelReturn {
    let Some(arg) = args().nth(1) else {
        return HWheelReturn::ForgotArgument;
    };
    match &*arg {
        "overlay" => bring_up_overlay(),
        "scrollup" => scrollup(),
        "scrolldown" => scrolldown(),
        "click" => {
            let Some(b) = args().nth(2) else {
                return HWheelReturn::NoClickButton;
            };
            let Ok(b) = b.parse() else {
                return HWheelReturn::ClickButtonIsntNumber;
            };
            click(b);
        }
        _ => {}
    }
    HWheelReturn::Success
    //bring_up_overlay();
}
