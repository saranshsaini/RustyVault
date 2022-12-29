use super::{Input, NavigationResult, Page, PageResult, PasswordManager};
use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

impl PasswordManager {
    pub fn home_screen<T: tui::backend::Backend>(
        &self,
        terminal: &mut Terminal<T>,
    ) -> NavigationResult {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                .split(size);

            let tabs = self.get_header(vec!["Home", "Passwords List"]);
            rect.render_widget(tabs, chunks[0]);
            rect.render_widget(self.render_home(), chunks[1])
        })?;
        loop {
            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('p') => return Ok(Page::PasswordList),
                    _ => {}
                },
                Input::Noop => {}
            }
        }
        Ok(Page::Quit)
    }

    fn render_home<'a>(&self) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "The RustyBox Password Manager",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Press 'p' to access password list.")]),
            Spans::from(vec![Span::raw("'q' to exit.")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }
}
