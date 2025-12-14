#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod task;
mod ui;

use ping::Ping;
use std::{sync::mpsc, thread};

use macroquad::{
    miniquad::{conf::Icon, native},
    prelude::*,
};

use crate::{
    config::Config,
    task::{Message, Status, Update},
    ui::{ARIAL_PATH, BG, PADDING, Settings},
};

fn conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: 600,
        window_height: 400,
        high_dpi: true,
        // TODO: icon
        icon: None,

        window_title: Config::read()
            .map(|c| c.window_title)
            .unwrap_or("pingky".to_string()),
        ..Default::default()
    }
}

pub fn popup_err(title: &str, message: &str) {
    if let Err(e) = msgbox::create(title, message, msgbox::IconType::Error) {
        eprintln!("failed to show message box: {e}");
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut font = load_ttf_font(ARIAL_PATH).await.unwrap();

    let config = if Config::exists() {
        println!("config exists: trying to read...");
        match Config::read() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                popup_err("ERROR: failed to read config", &e.to_string());
                return;
            }
        }
    } else {
        println!("config does not exist: creating...");
        let c = Config::default();
        if let Err(e) = c.write() {
            let title = "ERROR: failed to write config";
            popup_err(title, &e.to_string());
            return;
        }
        c
    };

    println!("config = {config:#?}");

    // pre-calculate UI
    let title_size = measure_text(&config.window_title, Some(&font), ui::SIZE_LARGE, 1.0);
    let title_pos = vec2(
        screen_width() / 2.0 - title_size.width / 2.0,
        ui::SIZE_LARGE as f32,
    );

    let font_normal_height = measure_text("Hello", Some(&font), ui::SIZE_NORMAL, 1.0).height;

    // settings
    let mut settings = Settings::new(&config);
    ui::apply_skin(&font);

    // list of responses
    let mut status = vec![Status::InProgress; config.targets.len()];

    // start up the ping threads
    let (tx, rx) = mpsc::channel::<Message>();

    let mut pinger = Ping::new(*config.targets.values().next().unwrap());
    pinger.ttl(5);

    let mut i = 0;
    for (_name, addr) in &config.targets {
        let tx = tx.clone();
        let addr = addr.clone();
        thread::spawn(move || task::start(i, addr, config.ping_interval, tx.clone()));
        i += 1;
    }

    loop {
        // update
        while let Ok((index, update)) = rx.try_recv() {
            let (name, addr) = config.targets.iter().nth(index).unwrap();
            println!("[{index} : {name}] {addr}\t=> {update:?}");

            match update {
                Update::Starting => {
                    status[index] = Status::InProgress;
                }
                Update::Ok { latency_ms } => {
                    status[index] = Status::Ok;
                }
                Update::Err(_e) => {
                    status[index] = Status::Failed;
                }
            }
        }

        // draw
        clear_background(BG);

        draw_text_ex(
            &config.window_title,
            title_pos.x,
            title_pos.y,
            TextParams {
                font: Some(&font),
                font_size: ui::SIZE_LARGE,
                color: BLACK,
                ..Default::default()
            },
        );

        let mut x = 50.0;
        let mut y = 70.0;
        let r = 10.0;

        let offset = 30.;
        let font_size = ui::SIZE_NORMAL;

        for (i, (name, _addr)) in config.targets.iter().enumerate() {
            let color = status[i].color();
            draw_circle(x, y, r, color);

            draw_text_ex(
                name,
                x + r + PADDING,
                y + font_normal_height / 2.0,
                TextParams {
                    font: Some(&font),
                    font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );
            y += offset;
        }

        // settings overlay
        // settings.show();

        next_frame().await;
    }
}
