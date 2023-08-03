/*
 * TODO:
 * + display text from file;
 * - display the cursor;
 * - blinking cursor;
 * - add font;
 * - edit text & save it back to the file;
 * - pass file name as a parameter;
 * - menu bar: change the font size;
 * - open other file in the same dir;
 * - settings file: font size;
 * - settings file: window dims;
 * - set cursor pos with mouse;
 * - text selection with mouse;
 */

use raylib::{prelude::*, ffi::{GuiSetFont, LoadFontEx, Font}};

fn get_file_content(file_path: String) -> String {
    std::fs::read_to_string(&file_path).expect(&format!("can't read {file_path} file"))
}

/// Responsible for updating app state
fn update(rl: &RaylibHandle, cursor: &mut Rectangle ) {

    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT) {
        cursor.x += 30.;
    }

    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) {
        cursor.x -= 30.;
    }
}

/// Responsible for visual representation of the current state of the app
fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, text: &str, font_size: i32, cursor: &Rectangle, offset_x: i32, offset_y: i32, font: Font) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    d.draw_rectangle(cursor.x as i32, cursor.y as i32, cursor.width as i32, cursor.height as i32, Color::WHITE);
    let mut line_num = 0;
    let mut text_offset_x = offset_x;


    for (i, c) in text.chars().enumerate() {
        if c == '\n' {
            line_num += 1;
            text_offset_x = offset_x;
            continue;
        }
        d.draw_text(&c.to_string(), text_offset_x, offset_y + line_num * font_size, 40, Color::GRAY);
        text_offset_x += font_size;
    }
}

fn main() {

    // create raylib window
    const WINDOW_WIDTH: i32 = 2000;
    const WINDOW_HEIGHT: i32 = 1200;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("notebook")
        .build();

    rl.set_target_fps(30);

    // load font
    let file_path: &str = "JetBrains_Mono.ttf";
    let file_path_ptr: *const i8 = file_path.as_ptr() as *const i8;
    let mut chars = 0;
    let font: Font;


    unsafe {
        // define a font base size of 30 pixels tall and up to 250 characters
        font = LoadFontEx(file_path_ptr, 30, &mut chars, 250);
        GuiSetFont(font);
    }


    // init editor state
    let font_size = 40;
    let offset_x = 10.;
    let offset_y = 20.;

    let mut cursor = Rectangle {
        x: offset_x,
        y: offset_y,
        width: 10.,
        height: font_size as f32,
    };

    let text = get_file_content("test.txt".to_string());
    while !rl.window_should_close() {
        update(&rl, &mut cursor);
        draw(&mut rl, &thread, &text, font_size, &cursor, offset_x as i32, offset_y as i32, font);
    }
}
