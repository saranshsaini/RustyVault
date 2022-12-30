use super::{Input, InputResult, NavigationResult, Page, PasswordManager};
use crossterm::event::KeyCode;
use directories::ProjectDirs;
use std::fs;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

impl PasswordManager {
    pub fn init_screen<T: tui::backend::Backend>(
        &self,
        terminal: &mut Terminal<T>,
    ) -> NavigationResult {
        if self.is_initialized() {
            return Ok(Page::Home);
        }
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Min(2)].as_ref())
                .split(size);

            // let tabs = self.get_header(vec!["Home", "Passwords List"]);
            // rect.render_widget(tabs, chunks[0]);
            rect.render_widget(self.render_init(), chunks[0])
        })?;
        loop {
            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Enter => {
                        let page_res = self.user_input(
                            terminal,
                            "Create Login Password. At least 5 characters long.",
                        )?;
                        if page_res.input.len() < 5 {
                            return Ok(Page::Initialize);
                        }
                        // let raw_pw = page_res.input;
                        // let enc_login_pw = self.security.encrypt_login_pw(&page_res.input[..]);
                        self.db.save_login_pw(&page_res.input[..]);
                        return Ok(Page::Home);
                    }
                    _ => {}
                },
                Input::Noop => {}
            }
        }
        Ok(Page::Quit)
    }

    fn render_init<'a>(&self) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::styled(
                "The RustyBox Password Manager Initializer",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(
                "To begin using RustyBox, press 'enter' to create a password.",
            )]),
            Spans::from(vec![Span::raw("Don't forget it!.")]),
            Spans::from(vec![Span::raw("'q' to exit.")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Initialize New User")
                .border_type(BorderType::Plain),
        );
        home
    }

    fn is_initialized(&self) -> bool {
        let proj_dirs = ProjectDirs::from("com", "RustyBoxTeam", "RustyBox")
            .expect("Could not access home folder from OS");
        let dir_path = proj_dirs.config_dir();
        let keys_path = dir_path.join("keys.json");
        let data_path = dir_path.join("data.json");
        if !dir_path.exists() {
            fs::create_dir(dir_path).expect("Can not create config folder");
        }
        if !keys_path.exists() || !data_path.exists() {
            fs::File::create(keys_path).expect("Can not create key file");
            fs::File::create(data_path).expect("Can not create data file");
            return false;
        }
        if fs::read_to_string(keys_path).unwrap().is_empty()
        //|| fs::read_to_string(data_path).unwrap().is_empty()
        {
            return false;
        }
        return true;
    }
}
