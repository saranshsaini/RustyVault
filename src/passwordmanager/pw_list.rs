use super::{Input, InputResult, NavigationResult, Page, PasswordManager};
use std::cmp;

use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
    Terminal,
};
impl PasswordManager {
    pub fn pw_list_screen<T: tui::backend::Backend>(
        &self,
        terminal: &mut Terminal<T>,
    ) -> NavigationResult {
        let mut pw_list_state = ListState::default();
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
                        "'enter' - show password. 'c' - copy to clipboard.",
                    )]),
                    Spans::from(vec![Span::raw(
                        "'a' - add. 'e' - edit. 'd' - delete. 'h' - home",
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
                // rect.render_stateful_widget(left, pw_chunks[0], &mut pw_list_state);
                if self.db.pw_vec.is_empty() {
                    rect.render_widget(empty_message, chunks[1]);
                } else {
                    let pw_table = self.render_pwlist(&mut pw_list_state);
                    rect.render_widget(pw_table, chunks[1]);
                }
                rect.render_widget(instructions, chunks[2]);

                // rect.render_widget(self.render_pwlist(&pw_list_state), chunks[1])
            })?;

            match self.input_rx.recv()? {
                Input::Key(k) => match k.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('h') => return Ok(Page::Home),
                    KeyCode::Up => {
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let prev = pw_list_state.selected().unwrap();

                        pw_list_state.select(Some(
                            prev.checked_sub(1).unwrap_or(self.db.pw_vec.len() - 1),
                        ));
                    }
                    KeyCode::Down => {
                        if self.db.pw_vec.is_empty() {
                            continue;
                        }
                        let prev = pw_list_state.selected().unwrap();
                        // pw_list_state.select(Some(cmp::min(self.db.pw_vec.len() - 1, prev + 1)));
                        pw_list_state.select(Some((prev + 1) % self.db.pw_vec.len()));
                    }

                    // KeyCode::Char('p') => return Ok(Page::PasswordList),
                    _ => {}
                },
                Input::Noop => {}
            }
        }
        Ok(Page::Quit)
    }
    fn render_pwlist<'a>(&self, pw_list_state: &mut ListState) -> (Table<'a>) {
        // let pets = Block::default()
        //     .borders(Borders::ALL)
        //     .style(Style::default().fg(Color::White))
        //     .title("Sites")
        //     .border_type(BorderType::Plain);

        let pw_list = &self.db.pw_vec;
        // if pw_list.is_empty(){
        //     return Table::new(vec!)
        // }

        let pw_index = pw_list_state.selected().unwrap();
        // pw_index = if pw_index >= pw_list.len() {
        //     0
        // } else {
        //     pw_index
        // };
        // pw_list_state.select(Some(pw_index));

        let selected_site_id = pw_list.get(pw_index).unwrap().id;

        let rows = pw_list.iter().map(|pw| {
            let id_span = if selected_site_id == pw.id {
                Span::styled(
                    pw.id.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(pw.id.to_string())
            };
            Row::new(vec![
                Cell::from(id_span),
                Cell::from(Span::raw(pw.site.clone())),
                Cell::from(Span::raw(pw.username.clone())),
                Cell::from(Span::raw(pw.password.clone())),
                Cell::from(Span::raw(pw.created.to_string())),
                Cell::from(Span::raw(pw.last_updated.to_string())),
            ])
        });
        let pw_table = Table::new(rows)
            .header(Row::new(vec![
                Cell::from(Span::styled(
                    "ID",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    "Site",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    "Username",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    "Password",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    "Created",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Cell::from(Span::styled(
                    "Last Updated",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
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
