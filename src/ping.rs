use std::thread::{self, Thread};

use crate::config::Config;

struct Instance {
    index: usize,
}
impl Instance {
    pub fn spawn(index: usize) {}
}

pub struct Ping<'a> {
    threads: Vec<Thread>,
    config: &'a Config,
}

impl<'a> Ping<'a> {
    pub fn new(config: &'a Config) -> Self {
        todo!()
    }
}
