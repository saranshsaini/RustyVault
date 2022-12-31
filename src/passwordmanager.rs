use super::databasemanager::DatabaseManager;
use chrono::{Local, NaiveDate};
use crossterm::{
    event::{self, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::{
    io, thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};
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

pub struct PasswordManager {
    input_rx: mpsc::Receiver<Input<KeyEvent>>,
    page: Page,
    db: DatabaseManager,
    db_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordEntry {
    username: String,
    pub password: String,
    site: String,
    created: NaiveDate,
    pub last_updated: NaiveDate,
}

impl PasswordEntry {
    pub fn new(site: String, username: String, enc_pw: String) -> PasswordEntry {
        PasswordEntry {
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
        let (input_rx, _) = PasswordManager::start_input_thread();
        PasswordManager {
            input_rx,
            page: Page::Initialize,
            db: DatabaseManager::new(),
            db_key: String::new(),
        }
    }
    pub fn show(&mut self) -> Error {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        let mut message = "Enter Password to See Passwords List";
        let mut authenticated = false;
        loop {
            match self.page {
                Page::Initialize => self.page = self.init_screen(&mut terminal)?,
                Page::Home => {
                    authenticated = false;
                    message = "Enter Password to See Passwords List";
                    self.page = self.home_screen(&mut terminal)?;
                }
                Page::Quit => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                Page::PasswordList => {
                    if !authenticated {
                        let page_res = self.user_input(&mut terminal, message)?;
                        if let Page::Home = page_res.page {
                            self.page = Page::Home;
                            continue;
                        }
                        if !self.db.verify_login_pw(&page_res.input) {
                            message = "Incorrect Password. Try Again";
                            continue;
                        }
                        self.db_key = self.db.get_db_key(&page_res.input);
                        self.db.populate_data(&self.db_key);
                    }
                    authenticated = true;
                    self.page = self.pw_list_screen(&mut terminal)?;
                }
            }
        }

        disable_raw_mode()?;
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

    //use parameter
    pub fn get_header(&self, _: Vec<&str>) -> Tabs {
        let menu_titles = vec!["Home", "Passwords List"];
        // let menu_titles = menu_titles.clone();
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
