use hamsterwheel::{
    HWheelError, KeyDistribution, CHILD_GAP, GRID_HEIGHT, GRID_WIDTH, HAMSTER_BACKGROUND,
    HAMSTER_OPACITY, LOCKED_RECT_COLOR, LOCKED_RECT_SIDE, PADDING_H, PADDING_W, TARGET_FPS,
    TEXT_COLOR,
};
use raylib::prelude::*;

const KEYS: KeyDistribution<{ GRID_WIDTH as usize }> = KeyDistribution::new(
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ñ'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-'],
);

#[derive(Debug, Default)]
enum OverlayKind {
    /// When it's first opened up and we're selecting the region
    #[default]
    Selecting,
    /// After a region is selected, if we want to specify.
    Specifying,
}

#[derive(Debug, Default)]
struct OverlayState {
    is_locked: bool,
    kind: OverlayKind,
}

pub fn bring_up_overlay() -> Result<(), HWheelError> {
    let (mut rl, thread) = raylib::init().title("Hamster").build();

    rl.set_window_opacity(HAMSTER_OPACITY);
    rl.toggle_borderless_windowed();

    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_target_fps(TARGET_FPS);

    let curr_mon = get_current_monitor();
    let (mon_w, mon_h) = (get_monitor_width(curr_mon), get_monitor_height(curr_mon));

    let cell_width = mon_w / GRID_WIDTH;
    let cell_height = mon_h / GRID_HEIGHT;
    let font_size = cell_height;

    let uiua386 = rl
        .load_font_ex(&thread, "../assets/Uiua386.ttf", cell_height, None)
        .expect("could not find uiua386");

    let mut state = OverlayState::default();
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            state.is_locked = !state.is_locked;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(HAMSTER_BACKGROUND);

        for i in 0..GRID_HEIGHT {
            d.draw_line_ex(
                Vector2::new(0 as f32, (cell_height * i) as f32),
                Vector2::new(mon_w as f32, (cell_height * i) as f32),
                CHILD_GAP,
                Color::BLACK,
            );
        }
        for j in 0..GRID_WIDTH {
            d.draw_line_ex(
                Vector2::new((cell_width * j) as f32, 0 as f32),
                Vector2::new((cell_width * j) as f32, mon_h as f32),
                CHILD_GAP,
                Color::BLACK,
            );
        }

        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                d.draw_text_ex(
                    &uiua386,
                    // TODO: figure out which keys to show
                    &KEYS.get(0, j).unwrap_or('?').to_uppercase().to_string(),
                    Vector2::new(
                        (j * cell_width + PADDING_W + font_size / 4) as f32,
                        (i * cell_height + PADDING_H) as f32,
                    ),
                    font_size as f32,
                    0.0,
                    TEXT_COLOR,
                );
                d.draw_text_ex(
                    &uiua386,
                    &KEYS
                        .get(i % 2 + 1, i / 2)
                        .unwrap_or('?')
                        .to_uppercase()
                        .to_string(),
                    Vector2::new(
                        (j * cell_width + PADDING_W + 3 * font_size / 4) as f32,
                        (i * cell_height + PADDING_H) as f32,
                    ),
                    font_size as f32,
                    0.0,
                    TEXT_COLOR,
                );
            }
        }

        if state.is_locked {
            d.draw_rectangle(
                0,
                mon_h - LOCKED_RECT_SIDE,
                LOCKED_RECT_SIDE,
                LOCKED_RECT_SIDE,
                LOCKED_RECT_COLOR,
            );
        }
    }
    Ok(())
}
