use macroquad::prelude::*;

pub const BG: Color = Color::from_hex(0xf1f4f7);

pub const ARIAL_PATH: &str = if cfg!(target_os = "windows") {
    r"C:\Windows\Fonts\arial.ttf"
} else {
    panic!("TODO: font paths")
};

/* UI font sizes */

pub const SIZE_NORMAL: u16 = 25;
pub const SIZE_LARGE: u16 = 40;

pub const PADDING: f32 = 25.0;

pub fn screen_rect() -> Rect {
    Rect {
        x: 0.0,
        y: 0.0,
        w: screen_width(),
        h: screen_height(),
    }
}

pub fn screen_size() -> (f32, f32) {
    (screen_width(), screen_height())
}
