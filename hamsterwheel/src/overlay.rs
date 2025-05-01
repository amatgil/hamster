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

#[derive(Debug)]
enum OverlayState {
    /// When it's first opened up and we're selecting the region
    Selecting { key_seq: Vec<char> },
    /// After a region is selected, if we want to specify.
    /// Only available if it's locked
    Specifying {
        cell_y: i32,
        cell_x: i32,
        spec_key_seq: Vec<char>,
    },
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

    let mut state = OverlayState::Selecting { key_seq: vec![] };

    let mut queued_up_click = None;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            // It's queued up because the click must occur after the overlay closes
            queued_up_click = Some(match rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
                false => "1",
                true => "3",
            });
            break;
        }

        match state {
            OverlayState::Selecting { ref mut key_seq } => {
                if let Some(key) = rl.get_char_pressed().filter(|k| !['l', ' '].contains(k)) {
                    key_seq.push(key);
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
                    &key_seq,
                );
                if key_seq.len() == 2 {
                    match moveto_dest {
                        Some((y_window, x_window)) => {
                            moveto(
                                (y_window * cell_height + cell_height / 2) as usize,
                                (x_window * cell_width + cell_width / 2) as usize,
                            )?;
                            state = OverlayState::Specifying {
                                cell_y: y_window,
                                cell_x: x_window,
                                spec_key_seq: vec![],
                            };
                        }
                        _ => unreachable!("Error in draw_grid_letters, probably"),
                    }
                }
            }
            OverlayState::Specifying {
                cell_y,
                cell_x,
                ref spec_key_seq,
            } => {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(HAMSTER_BACKGROUND);

                draw_grid_lines(&mut d, cell_height, mon_w, cell_width, mon_h);
                draw_smaller_grid_lines(&mut d, cell_height, cell_y, cell_width, cell_x);
                draw_smaller_grid_letters(
                    &mut d,
                    &uiua386,
                    cell_width,
                    cell_height,
                    cell_y,
                    cell_x,
                    font_size,
                    2,
                );
            }
        }
    }

    std::mem::drop(thread);
    std::mem::drop(rl); // TODO: This segfaults when main's scope ends. Fix
    sleep(Duration::from_millis(20));
    if let Some(c) = queued_up_click {
        eprintln!("CLICKING BUTTON: {c}");
        click(c)?;
    }
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

fn draw_smaller_grid_lines(
    d: &mut RaylibDrawHandle,
    cell_height: i32,
    base_cell_y: i32,
    cell_width: i32,
    base_cell_x: i32,
) {
    let base_y = base_cell_y * cell_height;
    let base_x = base_cell_x * cell_width;
    for dx in 0..=3 {
        d.draw_line_ex(
            Vector2::new((base_x + cell_width / 3 * dx) as f32, base_y as f32),
            Vector2::new(
                (base_x + cell_width / 3 * dx) as f32,
                (base_y + cell_height) as f32,
            ),
            CHILD_GAP,
            Color::BLACK,
        );
    }
    for dy in 0..3 {
        d.draw_line_ex(
            Vector2::new(base_x as f32, (base_y + cell_height / 3 * dy) as f32),
            Vector2::new(
                (base_x + cell_width) as f32,
                (base_y + cell_height / 3 * dy) as f32,
            ),
            CHILD_GAP,
            Color::BLACK,
        );
    }
}
fn draw_smaller_grid_letters(
    d: &mut RaylibDrawHandle,
    uiua386: &Font,
    cell_width: i32,
    cell_height: i32,
    grid_y: i32,
    grid_x: i32,
    font_size: i32,
    recursion_level: i32,
) {
    let s = 3i32.pow(recursion_level as u32);
    let font_size = font_size / (2 * recursion_level);

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            let text_x = grid_x * cell_width + j * cell_width / s + cell_width / s / 2;
            let text_y = grid_y * cell_height + i * cell_height / s - cell_height / s / 2;

            d.draw_text_ex(
                uiua386,
                &KEYS.get(i, j + 5).unwrap_or('?').to_string(),
                Vector2::new(text_x as f32, text_y as f32),
                font_size as f32,
                0.0,
                TEXT_COLOR,
            )
        }
    }
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
