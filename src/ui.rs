use crate::menu::Menu;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, menu: &Menu) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Draw prompt
    let prompt = Paragraph::new(menu.prompt.as_str())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(prompt, chunks[0]);

    // Draw menu items
    let items: Vec<ListItem> = menu
        .filtered_items
        .iter()
        .enumerate()
        .map(|(i, &idx)| {
            let item = &menu.items[idx];
            let style = if i == menu.selected {
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![Span::styled(&item.label, style)]))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    f.render_widget(list, chunks[1]);

    // Draw filter
    let filter = Paragraph::new(format!("Filter: {}", menu.filter))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(filter, chunks[2]);
}
