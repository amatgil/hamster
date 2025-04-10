use raylib::{ffi::SetConfigFlags, prelude::*};

const HAMSTER_OPACITY: f32 = 0.2;
const HAMSTER_BACKGROUND: Color = Color {
    // TODO: Make this look good
    r: 62,
    g: 176,
    b: 60,
    a: 0,
};

/// Elements per row
const GRID_HEIGHT: i32 = 13;
/// Elements per column
const GRID_WIDTH: i32 = 13;

const CHILD_GAP: f32 = 2.5;

const PADDING_W: i32 = 0;
const PADDING_H: i32 = 0;

const FONT_SIZE: i32 = 100;
const TEXT_COLOR: Color = Color::BLACK;

const TARGET_FPS: u32 = 20;

fn main() {
    let keys = vec![
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
        vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
    ];

    assert!(keys.len() > 0);
    unsafe { ffi::SetConfigFlags(ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH as u32) };
    let (mut rl, thread) = raylib::init().title("Hamster").build();

    rl.set_window_opacity(HAMSTER_OPACITY);
    rl.toggle_borderless_windowed();

    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_target_fps(TARGET_FPS);

    while !rl.window_should_close() {
        let curr_mon = get_current_monitor();
        let (mon_w, mon_h) = (get_monitor_width(curr_mon), get_monitor_height(curr_mon));

        let cell_width = mon_w / GRID_WIDTH;
        let cell_height = mon_h / GRID_HEIGHT;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(HAMSTER_BACKGROUND);

        for i in 0..GRID_HEIGHT {
            let i = i as i32;
            d.draw_line_ex(
                Vector2::new(0 as f32, (cell_height * i) as f32),
                Vector2::new(mon_w as f32, (cell_height * i) as f32),
                CHILD_GAP,
                Color::BLACK,
            );
        }
        for j in 0..GRID_WIDTH {
            let j = j as i32;
            d.draw_line_ex(
                Vector2::new((cell_width * j) as f32, 0 as f32),
                Vector2::new((cell_width * j) as f32, mon_h as f32),
                CHILD_GAP,
                Color::BLACK,
            );
        }

        d.draw_text(
            &keys[0][0].to_string(),
            PADDING_W + FONT_SIZE / 2,
            PADDING_H + FONT_SIZE / 2,
            FONT_SIZE,
            TEXT_COLOR,
        );
    }
}
