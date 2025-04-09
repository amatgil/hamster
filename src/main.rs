use raylib::prelude::*;

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

const PADDING_W: usize = 10;
const PADDING_H: usize = 10;

fn main() {
    let keys = vec![
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
        vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
    ];

    let (mut rl, thread) = raylib::init().title("Hamster").build();

    rl.set_window_opacity(HAMSTER_OPACITY);
    rl.toggle_borderless_windowed();

    assert!(keys.len() > 0);

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

        //d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
