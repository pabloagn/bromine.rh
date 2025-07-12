use crate::{menu::Menu, types::*, ui};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;
use tui_input::backend::crossterm::EventHandler;

pub struct App {
    menu: Menu,
    should_quit: bool,
}

impl App {
    pub fn new(menu: Menu) -> Self {
        Self {
            menu,
            should_quit: false,
        }
    }

    pub async fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<Option<MenuItem>> {
        loop {
            terminal.draw(|f| ui::draw(f, &self.menu))?;

            if self.should_quit {
                break;
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => self.should_quit = true,
                        KeyCode::Enter => {
                            if let Some(item) = self.menu.get_selected() {
                                return Ok(Some(item.clone()));
                            }
                        }
                        KeyCode::Up => self.menu.previous(),
                        KeyCode::Down => self.menu.next(),
                        KeyCode::PageUp => self.menu.page_up(10),
                        KeyCode::PageDown => self.menu.page_down(10),
                        KeyCode::Home => {
                            self.menu.selected = 0;
                            self.menu.scroll_offset = 0;
                        }
                        KeyCode::End => {
                            if !self.menu.filtered_indices.is_empty() {
                                self.menu.selected = self.menu.filtered_indices.len() - 1;
                                self.menu.update_scroll();
                            }
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.should_quit = true;
                        }
                        _ => {
                            self.menu.input.handle_event(&Event::Key(key));
                            self.menu.update_filter();
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}
