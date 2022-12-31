use super::{Input, InputResult, Page, PasswordManager};
use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
type PageResult = Result<InputResult, Box<dyn std::error::Error>>;
impl PasswordManager {
    pub fn user_input<T: tui::backend::Backend>(
        &self,
        terminal: &mut Terminal<T>,
        header_message: &str,
    ) -> PageResult {
        let mut input = String::new();
        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                    .split(size);

                let header = Paragraph::new(vec![Spans::from(vec![Span::raw(header_message)])])
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Red))
                            .border_type(BorderType::Plain),
                    );

                rect.render_widget(header, chunks[0]);
                rect.render_widget(self.render_text_input(&input), chunks[1])
            })?;

            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Esc => {
                        return Ok(InputResult {
                            page: Page::Home,
                            input: String::new(),
                        });
                    }
                    KeyCode::Enter => {
                        if input.len() == 0 {
                            continue;
                        }
                        return Ok(InputResult {
                            page: Page::PasswordList,
                            input,
                        });
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => {}
                },
                Input::Noop => {}
            }
        }
    }
    fn render_text_input<'a>(&self, input: &'a String) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("'esc' to go cancel.")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(input)]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Thick),
        );
        home
    }
}
