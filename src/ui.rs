use macroquad::ui::{Skin, hash};
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets::Window},
};

use crate::config::Config;

pub const BG: Color = Color::from_hex(0xf1f4f7);

pub const ARIAL_PATH: &str = if cfg!(target_os = "windows") {
    r"C:\Windows\Fonts\arial.ttf"
    // r"C:\Users\kianh\AppData\Local\Microsoft\Windows\Fonts\HackNerdFont-Regular.ttf"
} else {
    panic!("TODO: font paths")
};

/* UI font sizes */

pub const SIZE_SMALL: u16 = 15;
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

pub struct Settings {
    pub active: bool,
}

impl Settings {
    pub fn new(config: &Config) -> Self {
        Self { active: false }
    }
    pub fn show(&mut self) {
        let (w, h) = screen_size();

        let padding = PADDING / 2.0;

        let window = Rect::new(padding, padding, w - 2. * padding, h - 2. * padding);

        Window::new(hash!(), window.point(), window.size())
            .label("settings")
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "yo!");
            });
    }
}

pub fn apply_skin(font: &Font) {
    let base_style = || root_ui().style_builder().with_font(&font).unwrap();

    let label_style = base_style().font_size(SIZE_NORMAL).build();

    let window_style = base_style().font_size(SIZE_SMALL).build();

    let window_titlebar_style = base_style().build();

    let skin = Skin {
        label_style,
        window_style,
        window_titlebar_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin);
}
