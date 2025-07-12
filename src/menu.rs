use crate::bash;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub label: String,
    pub command: Option<String>,
}

pub struct Menu {
    pub prompt: String,
    pub items: Vec<MenuItem>,
    pub filtered_items: Vec<usize>,
    pub selected: usize,
    pub filter: String,
}

impl Menu {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            items: vec![],
            filtered_items: vec![],
            selected: 0,
            filter: String::new(),
        }
    }

    pub async fn load_from_command(&mut self, command: &str) -> Result<()> {
        let output = bash::execute_command(command).await?;
        self.parse_menu_items(&output)?;
        self.update_filter();
        Ok(())
    }

    fn parse_menu_items(&mut self, output: &str) -> Result<()> {
        self.items.clear();

        for line in output.lines() {
            if line.trim().is_empty() || line.starts_with("---") {
                continue;
            }

            if let Some((label, command)) = line.split_once(':') {
                self.items.push(MenuItem {
                    label: label.trim().to_string(),
                    command: Some(command.trim().to_string()),
                });
            } else {
                self.items.push(MenuItem {
                    label: line.trim().to_string(),
                    command: None,
                });
            }
        }

        Ok(())
    }

    pub fn next(&mut self) {
        if !self.filtered_items.is_empty() {
            self.selected = (self.selected + 1) % self.filtered_items.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered_items.is_empty() {
            self.selected = if self.selected == 0 {
                self.filtered_items.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn filter_push(&mut self, c: char) {
        self.filter.push(c);
        self.update_filter();
    }

    pub fn filter_pop(&mut self) {
        self.filter.pop();
        self.update_filter();
    }

    fn update_filter(&mut self) {
        self.filtered_items = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.label
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
            })
            .map(|(i, _)| i)
            .collect();

        self.selected = 0;
    }

    pub fn get_selected(&self) -> Option<&MenuItem> {
        self.filtered_items
            .get(self.selected)
            .and_then(|&idx| self.items.get(idx))
    }
}
