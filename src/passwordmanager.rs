use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::sync::mpsc;
use std::{
    io,
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
mod home;
mod second;
type Error = Result<(), Box<dyn std::error::Error>>;
type PageError = Result<Page, Box<dyn std::error::Error>>;
#[derive(Copy, Clone, Debug)]

pub enum Page {
    Quit,
    Home,
    PasswordList,
}
impl From<Page> for usize {
    fn from(input: Page) -> usize {
        match input {
            Page::Home => 0,
            Page::PasswordList => 1,
            Page::Quit => 2,
        }
    }
}

const TICK_RATE: Duration = Duration::from_millis(200);
enum Input<T> {
    Key(T),
    Noop,
}

pub struct PasswordManager {
    input_rx: mpsc::Receiver<Input<KeyEvent>>,
    input_thread_handle: thread::JoinHandle<()>,
    page: Page,
}

impl PasswordManager {
    pub fn new() -> PasswordManager {
        let (input_rx, input_thread_handle) = PasswordManager::start_input_thread();
        PasswordManager {
            input_rx,
            input_thread_handle,
            page: Page::Home,
        }
    }
    pub fn show(&mut self) -> Error {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        loop {
            match self.page {
                Page::Home => {
                    self.page = self.home_screen(&mut terminal)?;
                }
                Page::Quit => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                Page::PasswordList => {
                    self.page = self.second_screen(&mut terminal)?;
                }
            }
        }

        Ok(())
    }

    fn start_input_thread() -> (mpsc::Receiver<Input<KeyEvent>>, thread::JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = TICK_RATE
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("Polling Failed") {
                    if let Event::Key(key) = event::read().expect("Failed to read key event ") {
                        sender
                            .send(Input::Key(key))
                            .expect("failed to send key through channel");
                    }
                }

                if last_tick.elapsed() >= TICK_RATE {
                    if let Ok(_) = sender.send(Input::Noop) {
                        last_tick = Instant::now();
                    }
                }
            }
        });
        (receiver, handle)
    }
}
