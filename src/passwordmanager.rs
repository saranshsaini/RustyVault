use super::databasemanager::DatabaseManager;
use argon2::{self, Config};
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::{
    fs, io,
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
use uuid::Uuid;
mod home;
mod init;
mod list_functionality;
mod pw_list;
mod user_input;

type Error = Result<(), Box<dyn std::error::Error>>;
type NavigationResult = Result<Page, Box<dyn std::error::Error>>;

const TICK_RATE: Duration = Duration::from_millis(200);

#[derive(Copy, Clone, Debug)]
pub enum Page {
    Quit,
    Home,
    PasswordList,
    Initialize,
}
impl From<Page> for usize {
    fn from(input: Page) -> usize {
        match input {
            Page::Home => 0,
            Page::PasswordList => 1,
            Page::Quit => 2,
            Page::Initialize => 3,
        }
    }
}

enum Input<T> {
    Key(T),
    Noop,
}

pub struct InputResult {
    page: Page,
    input: String,
}
impl InputResult {
    pub fn new(page: Page, input: String) -> InputResult {
        InputResult { page, input }
    }
}

pub struct PasswordManager {
    input_rx: mpsc::Receiver<Input<KeyEvent>>,
    input_thread_handle: thread::JoinHandle<()>,
    page: Page,
    db: DatabaseManager,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordEntry {
    uuid: Uuid,
    username: String,
    password: String,
    site: String,
    created: NaiveDate,
    last_updated: NaiveDate,
}

impl PasswordEntry {
    pub fn new(site: String, username: String, enc_pw: String, uuid: Uuid) -> PasswordEntry {
        PasswordEntry {
            uuid,
            username,
            password: enc_pw,
            site,
            created: Local::now().date_naive(),
            last_updated: Local::now().date_naive(),
        }
    }
}

impl PasswordManager {
    pub fn new() -> PasswordManager {
        let (input_rx, input_thread_handle) = PasswordManager::start_input_thread();
        PasswordManager {
            input_rx,
            input_thread_handle,
            page: Page::Initialize,
            db: DatabaseManager::new(),
        }
    }
    pub fn show(&mut self) -> Error {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        let mut message = "Enter Password to See Passwords List";
        loop {
            match self.page {
                Page::Initialize => self.page = self.init_screen(&mut terminal)?,
                Page::Home => {
                    message = "Enter Password to See Passwords List";
                    self.page = self.home_screen(&mut terminal)?;
                }
                Page::Quit => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                Page::PasswordList => {
                    // self.page = self.second_screen(&mut terminal)?;
                    let page_res = self.user_input(&mut terminal, message)?;
                    if page_res.input.len() == 0 {
                        self.page = Page::Home;
                        continue;
                    }
                    if !self.db.verify_login_pw(&page_res.input) {
                        message = "Incorrect Password. Try Again";
                        continue;
                    }
                    self.page = Page::PasswordList;
                    self.page = self.pw_list_screen(&mut terminal)?;
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
    pub fn get_header<'a>(&self, menu_titles: Vec<&str>) -> Tabs {
        let menu_titles = vec!["Home", "Passwords List"];
        let menu_titles = menu_titles.clone();
        let menu = menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.to_owned().split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.page.into())
            .block(
                Block::default()
                    .title("Rusty Box")
                    .style(
                        Style::default().fg(Color::Green), // .add_modifier(Modifier::SLOW_BLINK),
                    )
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));
        tabs
    }
}
