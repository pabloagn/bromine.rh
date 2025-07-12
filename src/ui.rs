use crate::{menu::Menu, types::*};
use ratatui::{
    layout::{Alignment as TuiAlignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, menu: &Menu) {
    let area = centered_rect(menu.config.width, menu.config.height, f.area());

    // Draw block with borders only (no background on the block itself)
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::Rgb(125, 61, 82))); // Border color

    f.render_widget(block.clone(), area);

    // Get the inner area and fill it with background color
    let inner_area = block.inner(area);
    // f.render_widget(Clear, inner_area); // Clear first
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(32, 19, 30))),
        inner_area,
    );

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Prompt
            Constraint::Min(1),    // Items
            Constraint::Length(3), // Input
        ])
        .split(inner_area);

    draw_prompt(f, menu, inner_layout[0]);
    draw_items(f, menu, inner_layout[1]);
    draw_input(f, menu, inner_layout[2]);
}

fn draw_prompt(f: &mut Frame, menu: &Menu, area: Rect) {
    let prompt = Paragraph::new(menu.config.prompt.as_str())
        .style(Style::default().fg(Color::Rgb(88, 110, 117))) // Prompt color
        .alignment(TuiAlignment::Left);
    f.render_widget(prompt, area);
}

fn draw_items(f: &mut Frame, menu: &Menu, area: Rect) {
    let visible_items = menu.get_visible_items();
    let selected_idx = menu.filtered_indices.get(menu.selected).copied();

    let items: Vec<ListItem> = visible_items
        .iter()
        .map(|(idx, item)| {
            let is_selected = selected_idx == Some(*idx);

            let style = if is_selected {
                Style::default()
                    .bg(Color::Rgb(70, 41, 65)) // Selection color
                    .fg(Color::Rgb(242, 242, 242)) // Selection text
            } else {
                Style::default().fg(Color::Rgb(242, 242, 242)) // Text color
            };

            // Build spans for each column
            let mut spans = vec![];

            for (i, col_text) in item.columns.iter().enumerate() {
                let col_config = &menu.config.columns[i];
                let padded = match col_config.align {
                    Alignment::Left => {
                        format!("{:<width$}", col_text, width = col_config.width as usize)
                    }
                    Alignment::Right => {
                        format!("{:>width$}", col_text, width = col_config.width as usize)
                    }
                    Alignment::Center => {
                        format!("{:^width$}", col_text, width = col_config.width as usize)
                    }
                };

                let col_style = if is_selected && i == 0 {
                    style.fg(Color::Rgb(142, 64, 87)) // Selection-match color
                } else {
                    style
                };

                spans.push(Span::styled(padded, col_style));
                spans.push(Span::raw(" ")); // Column separator
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, area);

    // Draw scrollbar if needed
    if menu.filtered_indices.len() > 15 {
        draw_scrollbar(f, area, menu.scroll_offset, menu.filtered_indices.len());
    }
}

fn draw_input(f: &mut Frame, menu: &Menu, area: Rect) {
    let input = Paragraph::new(menu.input.value())
        .style(Style::default().fg(Color::Rgb(242, 242, 242)))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(125, 61, 82)))
                .title("Filter")
                .title_style(Style::default().fg(Color::Rgb(88, 110, 117))),
        );
    f.render_widget(input, area);
}

fn draw_scrollbar(f: &mut Frame, area: Rect, offset: usize, total: usize) {
    let scrollbar_area = Rect {
        x: area.x + area.width - 1,
        y: area.y,
        width: 1,
        height: area.height,
    };

    let visible_ratio = 15.0 / total as f32;
    let scrollbar_height = (visible_ratio * area.height as f32).max(1.0) as u16;
    let scrollbar_position = (offset as f32 / total as f32 * area.height as f32) as u16;

    let scrollbar = Block::default().style(Style::default().fg(Color::Rgb(88, 110, 117)));

    let scrollbar_rect = Rect {
        x: scrollbar_area.x,
        y: scrollbar_area.y + scrollbar_position,
        width: 1,
        height: scrollbar_height,
    };

    f.render_widget(scrollbar, scrollbar_rect);
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height - height) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width - width) / 2),
        ])
        .split(popup_layout[1])[1]
}
