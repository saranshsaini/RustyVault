use super::{Input, NavigationResult, Page, PasswordManager};
use crossterm::event::KeyCode;
use std::fs;
use std::path::Path;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
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
            Spans::from(vec![Span::raw("")]),
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
        let data_path = Path::new(&self.db.db_file);
        let keys_path = Path::new(&self.db.key_file);
        let dir_path = data_path.parent().unwrap();
        if !dir_path.exists() {
            fs::create_dir(dir_path).expect("Can not create config folder");
        }
        if !keys_path.exists() || !data_path.exists() {
            fs::File::create(keys_path).expect("Can not create key file");
            fs::File::create(data_path).expect("Can not create data file");
            return false;
        }
        if fs::read_to_string(keys_path).unwrap().is_empty() {
            fs::File::create(data_path).expect("Can not create data file");
            return false;
        }
        return true;
    }
}
