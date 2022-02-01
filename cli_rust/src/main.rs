use std::sync::mpsc;
use std::{io};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use crossterm::terminal::{enable_raw_mode};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use crate::app::resources::layout::base::{BaseLayout, render_footer, render_tabs};
use crate::app::resources::layout::help::LayoutHelp;
use crate::app::resources::layout::main::LayoutMain;
use crate::app::resources::layout::show_process::LayoutShowProcess;
use crate::app::thread::listen_event::ThreadListenEvent;
use crate::app::thread::sender_event::ThreadSendEvent;

mod app;
mod database;

pub enum Event{
    Input(KeyEvent),
    Tick,
}

pub struct StateApp<T: Backend> {
    current_menu: Menu,
    terminal: Terminal<T>,
}
#[derive(Copy, Clone)]
pub enum Menu{
    Main,
    Help,
    ShowProcess,
}

fn main() -> Result<(), Box<dyn Error>> {

    let (tx, rx) = mpsc::channel();
    let rx= Rc::new(rx);
    ThreadSendEvent::handle(tx);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let state_app= Rc::new(
        RefCell::new(
            StateApp{
                current_menu: Menu::Main,
                terminal,
            }
        )
    );

    enable_raw_mode().expect("cant run in raw mode");
    state_app.borrow_mut().terminal.clear()?;

    loop {
        let current_menu= state_app.borrow_mut().current_menu;
        state_app.borrow_mut().terminal.draw(|frame| {
            match current_menu {
                Menu::Main=> {
                    let layout= LayoutMain::new();
                    layout.render(frame);
                }
                Menu::Help=> {
                    let layout= LayoutHelp::new();
                    layout.render(frame);
                }
                Menu::ShowProcess=> {
                    let layout= LayoutShowProcess::new();
                    layout.render_special_content(frame);
                }
            }
        })?;

        ThreadListenEvent::handle(rx.clone(), state_app.clone())?;
    }

}