use super::{Input, NavigationResult, Page, PageResult, PasswordManager};
use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};
type InputResult = Result<PageResult, Box<dyn std::error::Error>>;
impl PasswordManager {
    pub fn user_input<T: tui::backend::Backend>(
        &self,
        terminal: &mut Terminal<T>,
        header_message: &str,
    ) -> InputResult {
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
                // let top = vec![Spans::from(vec![Span::raw("STUFF")])];
                // let tabs = Tabs::new(top)
                //     // .select(self.page.into())
                //     .block(
                //         Block::default()
                //             .title("RustyBox Input Required")
                //             .borders(Borders::ALL),
                //     )
                //     .style(Style::default().fg(Color::White))
                //     .highlight_style(Style::default().fg(Color::Yellow));

                rect.render_widget(header, chunks[0]);
                rect.render_widget(self.render_text_input(&input), chunks[1])
                // match active_menu_item {
                //     Page::Home => rect.render_widget(self.render_home(), chunks[1]),
                //     Page::PasswordList => rect.render_widget(self.render_list(), chunks[1]),
                // }
            })?;

            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Esc => {
                        return Ok(PageResult {
                            page: Page::Home,
                            input: String::new(),
                        });
                    }
                    KeyCode::Enter => {
                        if input.len() == 0 {
                            continue;
                        }
                        return Ok(PageResult {
                            page: Page::Home,
                            input,
                        });
                    }
                    // KeyCode::Char('h') => return Ok(Page::Home),
                    // KeyCode::Char('p') => return Ok(Page::PasswordList),
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => {}
                },
                Input::Noop => {}
            }
        }
        // Err()
    }
    fn render_text_input<'a>(&self, input: &'a String) -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("esc to go back.")]),
            Spans::from(vec![Span::raw("")]),
            // Spans::from(vec![Span::styled(
            //     "pet-CLI",
            //     Style::default().fg(Color::LightBlue),
            // )]),
            Spans::from(vec![Span::raw(input)]),
            // Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
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
