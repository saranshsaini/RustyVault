use super::{Input, NavigationResult, Page, PasswordManager};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, ListState, Paragraph, Row, Table},
    Terminal,
};
impl PasswordManager {
    pub fn pw_list_screen<T: tui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<T>,
    ) -> NavigationResult {
        let mut reveal = false;
        let mut pw_list_state = ListState::default();
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        pw_list_state.select(Some(0));
        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(15),
                            Constraint::Percentage(70),
                            Constraint::Percentage(15),
                        ]
                        .as_ref(),
                    )
                    .split(size);
                let tabs = self.get_header(vec!["Home", "Passwords List"]);
                let instructions = Paragraph::new(vec![
                    Spans::from(vec![Span::raw(
                        "'enter' - show password. 'c' - copy to clipboard. 'e' - edit password.",
                    )]),
                    Spans::from(vec![Span::raw(
                        "'a' - add. 'd' - delete. 'h' - home. 'q' - quit",
                    )]),
                ])
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Commands")
                        .border_type(BorderType::Plain),
                );

                let empty_message = Paragraph::new(vec![
                    Spans::from(vec![Span::raw("")]),
                    Spans::from(vec![Span::raw("No Passwords")]),
                ])
                .alignment(Alignment::Center);
                rect.render_widget(tabs, chunks[0]);
                if self.db.pw_vec.is_empty() {
                    rect.render_widget(empty_message, chunks[1]);
                } else {
                    let pw_table = self.render_pwlist(&mut pw_list_state, reveal);
                    rect.render_widget(pw_table, chunks[1]);
                }
                rect.render_widget(instructions, chunks[2]);
            })?;

            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('h') => return Ok(Page::Home),
                    KeyCode::Char('a') => return self.add_site(terminal),
                    KeyCode::Char('d') => {
                        reveal = false;
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let user_input = self
                            .user_input(terminal, "Type 'DELETE' to confirm deletion.")
                            .unwrap();
                        if user_input.input != "DELETE" {
                            continue;
                        }
                        self.db
                            .delete_site(pw_list_state.selected().unwrap(), &self.db_key);
                        pw_list_state.select(Some(0));
                    }
                    KeyCode::Char('e') => {
                        reveal = false;
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let user_input = self.user_input(terminal, "Enter new password").unwrap();
                        if user_input.input.is_empty() {
                            continue;
                        }
                        self.db.update_password(
                            pw_list_state.selected().unwrap(),
                            user_input.input,
                            &self.db_key,
                        )
                    }
                    KeyCode::Char('c') => {
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let _ = ctx.set_contents(
                            self.db
                                .pw_vec
                                .get(pw_list_state.selected().unwrap())
                                .unwrap()
                                .password
                                .to_owned(),
                        );
                    }
                    KeyCode::Up => {
                        reveal = false;
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let prev = pw_list_state.selected().unwrap();

                        pw_list_state.select(Some(
                            prev.checked_sub(1).unwrap_or(self.db.pw_vec.len() - 1),
                        ));
                    }
                    KeyCode::Down => {
                        reveal = false;
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let prev = pw_list_state.selected().unwrap();
                        // pw_list_state.select(Some(cmp::min(self.db.pw_vec.len() - 1, prev + 1)));
                        pw_list_state.select(Some((prev + 1) % self.db.pw_vec.len()));
                    }
                    KeyCode::Enter => {
                        reveal = !reveal;
                    }
                    _ => {}
                },
                Input::Noop => {}
            }
        }
        Ok(Page::Quit)
    }
    fn render_pwlist<'a>(&self, pw_list_state: &mut ListState, reveal: bool) -> Table<'a> {
        let pw_list = &self.db.pw_vec;
        let selected_site_id = pw_list_state.selected().unwrap();
        let mut curr_id = 0;
        let rows = pw_list.iter().map(|pw| {
            let mut id_span: Span = Span::raw(curr_id.to_string());
            let mut password: String = "*".repeat(pw.password.len());
            if selected_site_id == curr_id {
                id_span = Span::styled(
                    curr_id.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
                if reveal {
                    password = pw.password.clone();
                }
            }
            curr_id += 1;
            Row::new(vec![
                Cell::from(id_span),
                Cell::from(Span::raw(pw.site.clone())),
                Cell::from(Span::raw(pw.username.clone())),
                Cell::from(Span::raw(password)),
                Cell::from(Span::raw(pw.created.to_string())),
                Cell::from(Span::raw(pw.last_updated.to_string())),
            ])
        });
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Blue);
        let pw_table = Table::new(rows)
            .header(Row::new(vec![
                Cell::from(Span::styled("#", header_style)),
                Cell::from(Span::styled("Site", header_style)),
                Cell::from(Span::styled("Username", header_style)),
                Cell::from(Span::styled("Password", header_style)),
                Cell::from(Span::styled("Created", header_style)),
                Cell::from(Span::styled("Last Updated", header_style)),
            ]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Saved Sites")
                    .border_type(BorderType::Plain),
            )
            .widths(&[
                Constraint::Percentage(5),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]);

        pw_table
    }
}
