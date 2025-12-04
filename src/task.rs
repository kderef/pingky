use ping::Ping;
use std::{net::IpAddr, sync::mpsc::Sender, thread, time::Duration};

pub type Message = (usize, Result<Response, ping::Error>);

#[derive(Debug)]
pub struct Response {
    pub latency_ms: u64,
}

pub fn start(index: usize, target: IpAddr, interval: u64, tx: Sender<Message>) {
    let interval = Duration::from_secs(interval);

    let mut pinger = Ping::new(target);
    pinger.ttl(5);

    loop {
        let result = pinger.send().map(|r| Response {
            latency_ms: r.rtt.as_millis() as u64, // WARNING: down cast
        });

        if let Err(e) = tx.send((index, result)) {
            eprintln!("[task {index}] failed to send message: {e}");
        }

        thread::sleep(interval);
    }
}
