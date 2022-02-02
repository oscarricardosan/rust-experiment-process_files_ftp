use std::cell::RefCell;
use std::error::Error;
use std::process::exit;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use crossterm::event;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use tui::backend::{Backend};
use crate::{Action, Event, Menu, StateApp};

pub struct ThreadListenEvent {
}

impl ThreadListenEvent {
    pub fn handle<T: Backend>(
        rx: Rc<Receiver<Event>>,
        state_app: Rc<RefCell<StateApp<T>>>
    ) -> Result<(), Box<dyn Error>>  {

        match rx.recv()? {
            Event::Input(event) => match event {
                event::KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::ALT,
                }=> {
                    state_app.borrow_mut().current_menu= Menu::Help;
                }
                event::KeyEvent {
                    code: KeyCode::Char('v'),
                    modifiers: KeyModifiers::ALT,
                }=> {
                    state_app.borrow_mut().current_menu= Menu::ShowProcess;
                }
                event::KeyEvent {
                    code: KeyCode::Char('i'),
                    modifiers: KeyModifiers::ALT,
                }=> {
                    state_app.borrow_mut().current_menu= Menu::Main;
                }
                event::KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::ALT,
                } => {
                    disable_raw_mode()?;
                    state_app.borrow_mut().terminal.show_cursor()?;
                    state_app.borrow_mut().terminal.clear()?;
                    exit(0);
                }
                event::KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                } => {
                    let mut state_temp=state_app.borrow_mut();
                    if let Menu::ShowProcess = state_temp.current_menu{
                        state_temp.action= Some(Action::KeyDown);
                    }
                },
                event::KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                } => {
                    let mut state_temp=state_app.borrow_mut();
                    if let Menu::ShowProcess = state_temp.current_menu{
                        state_temp.action= Some(Action::KeyUp);
                    }
                },
                _ => {
                    // dbg!(event);
                }
            },
            Event::Tick => {

            }
        }

        Ok(())
    }
}
