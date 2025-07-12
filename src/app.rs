use crate::{menu::Menu, ui, Args};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, args: Args) -> Result<()> {
    let mut menu = Menu::new(&args.prompt);

    // Load menu items from bash command if provided
    if let Some(command) = args.command {
        menu.load_from_command(&command).await?;
    }

    loop {
        terminal.draw(|f| ui::draw(f, &menu))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => menu.previous(),
                    KeyCode::Down => menu.next(),
                    KeyCode::Enter => {
                        if let Some(selected) = menu.get_selected() {
                            // Execute the selected command
                            if let Some(cmd) = selected.command.as_ref() {
                                std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(cmd)
                                    .spawn()?;
                            }
                            break;
                        }
                    }
                    KeyCode::Char(c) => menu.filter_push(c),
                    KeyCode::Backspace => menu.filter_pop(),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
