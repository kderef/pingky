use ping::Ping;
use std::{
    net::IpAddr,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

use crate::{RUNNING, config};

pub struct Task<'a> {
    index: usize,
    timer: Instant,
    interval: u64,

    addr: IpAddr,
    ping: Ping<'a>,
}

impl<'a> Task<'a> {
    const SLEEP_DUR: Duration = Duration::from_millis(100);

    pub fn new(index: usize) -> Self {
        let timer = Instant::now();
        let conf = config::config();
        let interval = conf.ping_interval;
        let addr = conf.ping_targets[index].1;

        Self {
            index,
            timer,
            interval,
            addr,
            ping: ping::new(addr),
        }
    }

    pub fn start(&mut self) {
        while RUNNING.load(Ordering::Relaxed) {
            let elapsed = self.timer.elapsed();

            if elapsed.as_secs() >= self.interval {
                // ping
                println!("{} => PINGING {}", self.index, self.addr);

                self.timer = Instant::now();
            } else {
                thread::sleep(Self::SLEEP_DUR);
            }
        }
    }
}
