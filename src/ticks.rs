use async_std::task::sleep;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct Ticker {
    target: i16,
    tick: i16,
    started: bool,
}

impl Ticker {
    pub fn new() -> Self {
        Self {
            target: 0,
            tick: 0,
            started: false,
        }
    }

    pub fn reset(&mut self, seconds: i16) {
        self.target = seconds;
        self.tick = seconds;
        self.started = true;
    }

    pub fn remaining(&self) -> i16 {
        self.tick
    }

    pub fn completed(&self) -> bool {
        self.started && self.tick == 0
    }

    fn tick(&mut self) {
        if self.tick <= 0 {
            self.started = false;
            return;
        }

        self.tick -= 1;
    }
}

pub fn use_ticks() -> Signal<Ticker> {
    let mut ticker = use_signal(Ticker::new);
    use_future(move || async move {
        loop {
            ticker.write().tick();
            sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    ticker
}
