#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod task;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::{fs, process, thread};

use fltk::button::Button;
use fltk::dialog::{self, alert_default, message_default, message_icon};
use fltk::enums::{Align, Color, FrameType, Shortcut};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::input::Input;
use fltk::menu::MenuFlag;
use fltk::prelude::*;
use fltk::terminal::Terminal;
use fltk::utils::oncelock::OnceCell;
use fltk::{app::App, window::Window};

use crate::config::{CONFIG, Config, ConfigError};
use crate::task::Task;

#[derive(Clone, Copy)]
pub enum Status {
    Ok,
    Failed,
}

pub struct Message(usize, Status);

impl Status {
    pub const fn color(self) -> Color {
        match self {
            Self::Ok => Color::Green,
            Self::Failed => Color::Red,
        }
    }
}

pub static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    if let Err(e) = run() {
        let e = format!("{e}");
        eprintln!("{e}");
        alert_default(&e);
        process::exit(1);
    }
}

fn run() -> Result<(), ConfigError> {
    let config = match Config::read() {
        Ok(conf) => conf,
        Err(e) => {
            if !Path::new(Config::PATH).is_file() {
                println!("writing example to {}", Config::PATH);
                let conf = Config::write_example()?;

                let msg = format!(
                    "HELP: het bestand `{}` is aangemaakt.\nBewerk deze en herstart het programma.",
                    Config::PATH
                );
                message_default(&msg);

                conf
            } else {
                return Err(e);
            }
        }
    };
    let _ = CONFIG.set(config);

    // --- config successfuly loaded
    let config = config::config();
    let title = config.window_title.as_str();

    // --- open window
    let app = App::default().with_scheme(fltk::app::Scheme::Gtk);
    let mut wind = Window::new(0, 0, 500, 300, title).center_screen();
    wind.make_resizable(true);

    // --- menu bar
    let mut menubar = fltk::menu::SysMenuBar::new(0, 0, 400, 50, "Hello");
    menubar.set_frame(FrameType::DownFrame);
    menubar.add("Help", Shortcut::None, MenuFlag::Normal, |_| {
        println!("HELPQA");
    });
    menubar.end();

    // --- container for widgets
    let mut container = Flex::default_fill().column();
    container.set_margin(10);

    // --- title text
    let mut title_frame = Frame::default().with_label(title);
    title_frame.set_label_size(40);

    let row_height = 30;

    fltk::app::background(235, 235, 235);

    for (name, addr) in &config.ping_targets {
        let mut row = Flex::default().row();
        container.fixed(&row, row_height);

        // --- the colored ball on the left
        let mut indicator = Frame::new(0, 0, 100, 30, "");
        indicator.set_frame(FrameType::RoundUpBox);
        indicator.set_color(Color::Green);
        row.fixed(&indicator, 30);

        let mut label = Frame::default().with_label(&name);
        label.set_label_size(25);
        label.set_align(Align::Left | Align::Inside);

        container.fixed(&label, row_height);

        row.end();
    }

    container.fixed(&title_frame, 40);

    container.end();

    wind.end();
    wind.show();

    thread::scope(|scope| {
        for i in 0..config.ping_targets.len() {
            scope.spawn(move || {
                let mut task = Task::new(i);
                task.start();
            });
        }

        while app.wait() {}
        RUNNING.store(false, Ordering::Release);
    });

    app.run().unwrap();

    Ok(())
}
