use std::{thread::sleep, time::Duration};

use hamsterwheel::{
    click, moveto, HWheelError, KeyDistribution, CHILD_GAP, GRID_HEIGHT, GRID_WIDTH,
    HAMSTER_BACKGROUND, HAMSTER_OPACITY, LOCKED_RECT_COLOR, LOCKED_RECT_SIDE, PADDING_H, PADDING_W,
    TARGET_FPS, TEXT_COLOR,
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
    /// Only available if it's locked
    Specifying,
}

#[derive(Debug, Default)]
struct OverlayState {
    kind: OverlayKind,
    key_seq: Vec<char>,
}

pub fn bring_up_overlay() -> Result<(), HWheelError> {
    let (mut rl, thread) = raylib::init().title("Hamster").build();

    rl.set_window_opacity(HAMSTER_OPACITY);
    rl.toggle_borderless_windowed();

    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_target_fps(TARGET_FPS);

    let curr_mon = get_current_monitor();
    let (mon_w, mon_h) = (get_monitor_width(curr_mon), get_monitor_height(curr_mon));

    let (cell_width, cell_height) = (mon_w / GRID_WIDTH, mon_h / GRID_HEIGHT);
    let font_size = cell_height;

    let uiua386 = rl
        .load_font_ex(&thread, "../assets/Uiua386.ttf", cell_height, None)
        .expect("could not find uiua386");

    let mut state = OverlayState::default();

    let mut queued_up_click = None;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            queued_up_click = Some(match rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
                false => "1",
                true => "3",
            });
            break;
        }

        if let Some(key) = rl.get_char_pressed().filter(|k| !['l', ' '].contains(k)) {
            state.key_seq.push(key);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(HAMSTER_BACKGROUND);

        draw_grid_lines(&mut d, cell_height, mon_w, cell_width, mon_h);
        let moveto_dest = draw_grid_letters(
            &mut d,
            &uiua386,
            cell_width,
            font_size,
            cell_height,
            &state.key_seq,
        );

        match state.key_seq.len() {
            len if len == 2 => {
                if let Some((y_window, x_window)) = moveto_dest {
                    moveto(
                        (y_window * cell_height + cell_height / 2) as usize,
                        (x_window * cell_width + cell_width / 2) as usize,
                    )?;
                }
            }
            len if len > 2 => {
                println!("len is >2: {len}");
            }
            _ => {}
        }
    }

    std::mem::drop(thread);
    std::mem::drop(rl); // TODO: This segfaults when main's scope ends. Fix
    sleep(Duration::from_millis(20));
    if let Some(c) = queued_up_click {
        println!("CLICKING BUTTON: {c}");
        click(c)?;
    }
    println!("clicked!");
    sleep(Duration::from_millis(2000));
    Ok(())
}

fn keyseq_associated_to_cell(i: i32, j: i32) -> [char; 2] {
    [
        KEYS.get(0, j).unwrap_or('?').to_ascii_uppercase(),
        KEYS.get(i % 2 + 1, i / 2)
            .unwrap_or('?')
            .to_ascii_uppercase(),
    ]
}

fn draw_grid_letters(
    d: &mut RaylibDrawHandle,
    uiua386: &Font,
    cell_width: i32,
    font_size: i32,
    cell_height: i32,
    pressed_keys: &[char],
) -> Option<(i32, i32)> {
    let mut moveto_dest = None;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let curr_keyseq = keyseq_associated_to_cell(i, j);
            if pressed_keys.is_empty() || is_prefix_of(pressed_keys, &curr_keyseq) {
                if pressed_keys.len() == 2 {
                    moveto_dest = Some((i, j));
                }
                d.draw_text_ex(
                    uiua386,
                    &curr_keyseq[0].to_string(),
                    Vector2::new(
                        (j * cell_width + PADDING_W + font_size / 4) as f32,
                        (i * cell_height + PADDING_H) as f32,
                    ),
                    font_size as f32,
                    0.0,
                    TEXT_COLOR,
                );
                d.draw_text_ex(
                    uiua386,
                    &curr_keyseq[1].to_string(),
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
    }
    moveto_dest
}

fn draw_grid_lines(
    d: &mut RaylibDrawHandle,
    cell_height: i32,
    mon_w: i32,
    cell_width: i32,
    mon_h: i32,
) {
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
}

fn is_prefix_of(needle: &[char], haystack: &[char]) -> bool {
    if needle.len() > haystack.len() {
        false
    } else {
        for i in 0..needle.len() {
            if needle[i].to_ascii_uppercase() != haystack[i].to_ascii_uppercase() {
                return false;
            }
        }
        true
    }
}
