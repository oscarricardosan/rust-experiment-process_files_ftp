use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event;
use crate::{CrosstermEvent, Event};

pub struct ThreadSendEvent {

}

impl ThreadSendEvent {

    pub fn handle(tx:Sender<Event>) {

        thread::spawn(move || {
            let tick_rate = Duration::from_millis(200);
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("no poll works") {
                    if let CrosstermEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("cant send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });
    }

}