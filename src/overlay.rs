use std::{thread::sleep, time::Duration};

use hamster::{
    click, moveto, moveto_relative, HWheelError, KeyDistribution, CHILD_GAP, GRID_HEIGHT,
    GRID_WIDTH, HAMSTER_BACKGROUND, HAMSTER_OPACITY, LOCKED_RECT_COLOR, LOCKED_RECT_SIDE,
    OVERLAY_CLOSE_CLICK_DELAY_MS, PADDING_H, PADDING_W, TARGET_FPS, TEXT_COLOR,
};
use raylib::prelude::*;

const KEYS: KeyDistribution<{ GRID_WIDTH as usize }> = KeyDistribution::new(
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ñ'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-'],
);

const SPECIAL_KEYS: [char; 1] = [' '];

#[derive(Debug)]
enum OverlayState {
    /// When it's first opened up and we're selecting the region
    Selecting { key_seq: Vec<char> },
    /// After a region is selected, for further specification
    /// the base_{y, x} args are in absolute coordinates, no in cells, because they also indicate subcells (and subsubcells, etc.)
    Specifying {
        base_y: i32,
        base_x: i32,
        spec_key_seq: Vec<char>,
        /// To avoid repeatedly moving, only move when the lengths change.
        /// We must, then, store the last one
        len_of_last_move: Option<usize>,
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
        .load_font_ex(&thread, "assets/Uiua386.ttf", cell_height, None)
        .expect("could not find uiua386");

    let mut state = OverlayState::Selecting { key_seq: vec![] };

    let mut queued_up_click = None;
    while !rl.window_should_close() {
        let shift_down_rn = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            // It's queued up because the click must occur after the overlay closes
            queued_up_click = Some(match rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
                false => "1",
                true => "3",
            });
            break;
        }

        for (must_shift, k, dy, dx) in [
            // Not vim because the letters are already in use for grid-selection :(
            (false, KeyboardKey::KEY_LEFT, 0, -10),
            (false, KeyboardKey::KEY_RIGHT, 0, 10),
            (false, KeyboardKey::KEY_UP, -10, 0),
            (false, KeyboardKey::KEY_DOWN, 10, 0),
            (true, KeyboardKey::KEY_H, 0, -10),
            (true, KeyboardKey::KEY_J, 10, 0),
            (true, KeyboardKey::KEY_K, -10, 0),
            (true, KeyboardKey::KEY_L, 0, 10),
        ] {
            if rl.is_key_down(k) && must_shift == shift_down_rn {
                moveto_relative(dy, dx)?;
            }
        }

        match state {
            OverlayState::Selecting { ref mut key_seq } => {
                let last_pressed = rl.get_char_pressed().filter(|k| !SPECIAL_KEYS.contains(k));
                match (last_pressed, shift_down_rn) {
                    (Some(key), false) => key_seq.push(key),
                    (_, true) | (None, _) => {}
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
                    key_seq,
                );
                if key_seq.len() == 2 {
                    match moveto_dest {
                        Some((y_window, x_window)) => {
                            moveto(
                                (y_window * cell_height + cell_height / 2) as usize,
                                (x_window * cell_width + cell_width / 2) as usize,
                            )?;
                            state = OverlayState::Specifying {
                                base_y: y_window * cell_height,
                                base_x: x_window * cell_width,
                                spec_key_seq: vec![],
                                len_of_last_move: None,
                            };
                        }
                        _ => unreachable!("FATAL: Error in draw_grid_letters, probably"),
                    }
                }
            }
            OverlayState::Specifying {
                ref mut base_y,
                ref mut base_x,
                ref mut spec_key_seq,
                ref mut len_of_last_move,
            } => {
                let last_pressed = rl.get_char_pressed().filter(|k| !SPECIAL_KEYS.contains(k));
                let rec_level = spec_key_seq.len() as i32 + 1;

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(HAMSTER_BACKGROUND);

                draw_grid_lines(&mut d, cell_height, mon_w, cell_width, mon_h);
                draw_smaller_grid_lines(
                    &mut d,
                    cell_height,
                    *base_y,
                    cell_width,
                    *base_x,
                    rec_level,
                );
                let moveto_dest = draw_smaller_grid_letters(
                    &mut d,
                    &uiua386,
                    cell_width,
                    cell_height,
                    *base_y,
                    *base_x,
                    font_size,
                    last_pressed,
                    rec_level,
                );

                if let Some(((y, x), (i, j))) = moveto_dest {
                    if len_of_last_move.is_none()
                        || len_of_last_move.is_some_and(|l| l != spec_key_seq.len())
                    {
                        *len_of_last_move = len_of_last_move.map_or(Some(0), |l| Some(l + 1));
                        moveto(y as usize, x as usize)?;
                        *base_y = *base_y + cell_height / 3i32.pow(rec_level as u32) * i;
                        *base_x = *base_x + cell_width / 3i32.pow(rec_level as u32) * j;
                    }
                }
                match (last_pressed, dbg!(shift_down_rn)) {
                    (Some(key), false) => spec_key_seq.push(key),
                    (_, true) | (None, _) => {}
                }
            }
        }
    }

    std::mem::drop(rl); // TODO: This segfaults when main's scope ends. Fix
    sleep(Duration::from_millis(OVERLAY_CLOSE_CLICK_DELAY_MS));
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
            let curr_keyseq @ [left, right] = keyseq_associated_to_cell(i, j);
            if pressed_keys.is_empty() || is_prefix_of(pressed_keys, &curr_keyseq) {
                if pressed_keys.len() == 2 {
                    moveto_dest = Some((i, j));
                }
                d.draw_text_ex(
                    uiua386,
                    &left.to_string(),
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
                    &right.to_string(),
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
    base_y: i32,
    cell_width: i32,
    base_x: i32,
    recursion_level: i32,
) {
    let s = 3i32.pow(recursion_level as u32);

    for delta in 0..=3 {
        let bx = base_x + cell_width / s * delta;
        let by = base_y + cell_height / s * delta;
        d.draw_line_ex(
            Vector2::new(bx as f32, base_y as f32),
            Vector2::new(
                bx as f32,
                (base_y + cell_height / 3i32.pow(recursion_level as u32 - 1)) as f32,
            ),
            CHILD_GAP,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2::new(base_x as f32, by as f32),
            Vector2::new(
                (base_x + cell_width / 3i32.pow(recursion_level as u32 - 1)) as f32,
                by as f32,
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
    base_y: i32,
    base_x: i32,
    font_size: i32,
    last_pressed: Option<char>,
    recursion_level: i32,
) -> Option<((i32, i32), (i32, i32))> {
    let mut ret = None;
    let s = 3i32.pow(recursion_level as u32);
    let font_size = font_size / (2 * recursion_level);

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            let text_x = base_x + j * cell_width / s + cell_width / s / 2;
            let text_y = base_y + i * cell_height / s - cell_height / s / 2;
            let key_text = KEYS.get(i, j + 5).unwrap_or('?');

            d.draw_text_ex(
                uiua386,
                &key_text.to_string(),
                Vector2::new(text_x as f32, text_y as f32),
                font_size as f32,
                0.0,
                TEXT_COLOR,
            );
            if last_pressed.is_some_and(|l| l == key_text) {
                ret = Some((
                    ((text_y + font_size / (2 * recursion_level), text_x)),
                    (i, j),
                ))
            }
        }
    }
    ret
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
            if !needle[i].eq_ignore_ascii_case(&haystack[i]) {
                return false;
            }
        }
        true
    }
}
