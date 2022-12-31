use super::{Input, InputResult, NavigationResult, Page, PasswordManager};

use crossterm::event::KeyCode;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
    Terminal,
};

impl PasswordManager {
    pub fn add_site<T: tui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<T>,
    ) -> NavigationResult {
        let sitename_req = "Enter sitename. 'esc' to cancel.";
        let username_req = "Enter username. 'esc' to cancel.";
        let password_req = "Enter password. 'esc' to cancel.";
        let confirm_password_req = "Confirm Password. 'q' to cancel.";
        let sitename = self.user_input(terminal, sitename_req).unwrap().input;
        if sitename.is_empty() {
            return Ok(Page::Home);
        }
        let username = self.user_input(terminal, username_req).unwrap().input;
        if username.is_empty() {
            return Ok(Page::Home);
        }
        let mut password = self.user_input(terminal, password_req).unwrap().input;
        if password.is_empty() {
            return Ok(Page::Home);
        }
        let mut confirm_password = String::new();
        loop {
            if password.is_empty() {
                password = self.user_input(terminal, password_req).unwrap().input;
                if password.is_empty() {
                    return Ok(Page::Home);
                }
            }
            if confirm_password.is_empty() {
                confirm_password = self
                    .user_input(terminal, &confirm_password_req)
                    .unwrap()
                    .input;
                if confirm_password.is_empty() {
                    return Ok(Page::Home);
                }
            }
            if password == confirm_password {
                break;
            } else {
                password = String::new();
                confirm_password = String::new();
            }
        }
        self.db.add_pw(sitename, username, password, &self.db_key);
        Ok(Page::PasswordList)
    }
}
