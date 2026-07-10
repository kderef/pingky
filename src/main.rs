#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{
    app, button,
    image::{BmpImage, IcoImage},
    prelude::*,
    window::Window,
};
use fltk_theme::{
    ColorMap, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme, widget_themes,
};

const WINDOW_TITLE: &str = concat!("pingky v", env!("CARGO_PKG_VERSION"));
const WINDOW_W: i32 = 400;
const WINDOW_H: i32 = 200;

static ICON_BYTES: &[u8] = include_bytes!("../icon.bmp");

fn main() {
    let app = app::App::default();

    apply_themes(None, Some(ThemeType::Metro), None);

    let mut wind = Window::new(0, 0, WINDOW_W, WINDOW_H, WINDOW_TITLE).center_screen();

    let icon = BmpImage::from_data(ICON_BYTES).unwrap();
    wind.set_icon(Some(icon));

    let mut btn = button::Button::new(200, 100, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

    wind.end();
    wind.show();

    app.run().unwrap();
}

fn apply_themes(
    color_theme: Option<&[ColorMap]>,
    widget_theme: Option<ThemeType>,
    widget_scheme: Option<SchemeType>,
) {
    color_theme.map(|t| ColorTheme::new(t).apply());
    widget_theme.map(|t| WidgetTheme::new(t).apply());
    widget_scheme.map(|t| WidgetScheme::new(t).apply());
}
