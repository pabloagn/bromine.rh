use crate::types::*;
use anyhow::Result;
use tui_input::Input;

pub struct Menu {
    pub config: MenuConfig,
    pub items: Vec<MenuItem>,
    pub filtered_indices: Vec<usize>,
    pub selected: usize,
    pub input: Input,
    pub scroll_offset: usize,
}

impl Menu {
    pub fn new(config: MenuConfig) -> Self {
        let items = config.items.clone();
        let filtered_indices: Vec<usize> = (0..items.len()).collect();

        Self {
            config,
            items,
            filtered_indices,
            selected: 0,
            input: Input::default(),
            scroll_offset: 0,
        }
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let config: MenuConfig = serde_json::from_str(json)?;
        Ok(Self::new(config))
    }

    pub fn next(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected = (self.selected + 1) % self.filtered_indices.len();
            self.update_scroll();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected = if self.selected == 0 {
                self.filtered_indices.len() - 1
            } else {
                self.selected - 1
            };
            self.update_scroll();
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        if !self.filtered_indices.is_empty() {
            self.selected = (self.selected + page_size).min(self.filtered_indices.len() - 1);
            self.update_scroll();
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        if self.selected >= page_size {
            self.selected -= page_size;
        } else {
            self.selected = 0;
        }
        self.update_scroll();
    }

    pub fn update_scroll(&mut self) {
        let visible_height = 15; // Match your fuzzel config

        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected - visible_height + 1;
        }
    }

    pub fn update_filter(&mut self) {
        let filter = self.input.value().to_lowercase();

        self.filtered_indices = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                if filter.is_empty() {
                    return true;
                }

                // Check all columns for match
                item.columns
                    .iter()
                    .any(|col| col.to_lowercase().contains(&filter))
                    || item.id.to_lowercase().contains(&filter)
            })
            .map(|(i, _)| i)
            .collect();

        self.selected = 0;
        self.scroll_offset = 0;
    }

    pub fn get_selected(&self) -> Option<&MenuItem> {
        self.filtered_indices
            .get(self.selected)
            .and_then(|&idx| self.items.get(idx))
    }

    pub fn get_visible_items(&self) -> Vec<(usize, &MenuItem)> {
        let visible_height = 15;

        self.filtered_indices
            .iter()
            .skip(self.scroll_offset)
            .take(visible_height)
            .filter_map(|&idx| self.items.get(idx).map(|item| (idx, item)))
            .collect()
    }
}
