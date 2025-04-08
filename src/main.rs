use raylib::prelude::*;

const HAMSTER_OPACITY: f32 = 0.4;
const HAMSTER_BACKGROUND: Color = Color {
    // TODO: Make this look good
    r: 62,
    g: 176,
    b: 60,
    a: 0,
};

fn main() {
    let (mut rl, thread) = raylib::init().title("Hamster").build();

    rl.set_window_opacity(HAMSTER_OPACITY);
    rl.toggle_borderless_windowed();

    //let m = get_current_monitor_index();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(HAMSTER_BACKGROUND);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
