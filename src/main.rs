#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::draw::set_cursor;
use fltk::enums::{Align, Event};
use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::table::{Table, TableRow};
use fltk::{app, button, image::BmpImage, window::Window};
use fltk_table::SmartTable;
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

    let padding = 10;
    let table_height = WINDOW_H - padding * 2 - 20;
    let mut table = SmartTable::new(padding, padding, WINDOW_W - padding * 2, table_height, "")
        .with_opts(fltk_table::TableOpts {
            rows: 30,
            cols: 2,
            editable: true,
            header_frame: fltk::enums::FrameType::PlasticUpBox,
            ..Default::default()
        });

    table.set_col_resize(false);
    table.set_row_resize(false);

    // table.set_col_header_value(0, "Status");
    // table.set_col_header_value(1, "Target");

    table.set_col_width(1, 240);
    table.set_col_header_height(0);

    let mut t = table.clone();

    table.set_callback(|_| {
        set_cursor(fltk::enums::Cursor::Default);
    });

    table.set_on_update_callback(move |row, col, new_value| {
        println!("{row} {col} {new_value}");

        if col != 0 {
            t.set_cell_value(row, col, &new_value);
        }

        t.redraw();
    });

    wind.end();
    wind.show();

    // window events
    wind.set_callback(move |_| match app::event() {
        Event::Close => {
            app.quit();
        }
        _ => {}
    });

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
