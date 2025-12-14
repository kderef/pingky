use macroquad::prelude::*;
use ping::Ping;
use std::{net::IpAddr, sync::mpsc::Sender, thread, time::Duration};

pub type Message = (usize, Update);

#[derive(Debug)]
pub enum Update {
    Starting,
    Ok { latency_ms: u64 },
    Err(ping::Error),
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Ok,
    InProgress,
    Failed,
}
impl Status {
    pub const fn color(self) -> Color {
        match self {
            Self::Ok => GREEN,
            Self::InProgress => GRAY,
            Self::Failed => RED,
        }
    }
}

pub fn start(index: usize, target: IpAddr, interval: u64, tx: Sender<Message>) {
    let interval = Duration::from_secs(interval);

    let mut pinger = Ping::new(target);
    pinger.timeout(Duration::from_secs(4));

    let log_err = |e| eprintln!("[task {index}] failed to send message: {e}");

    loop {
        tx.send((index, Update::Starting)).unwrap_or_else(log_err);

        let result = match pinger.send() {
            Ok(r) => Update::Ok {
                latency_ms: r.rtt.as_millis() as u64, // WARNING: down cast
            },
            Err(e) => Update::Err(e),
        };

        tx.send((index, result)).unwrap_or_else(log_err);

        thread::sleep(interval);
    }
}
