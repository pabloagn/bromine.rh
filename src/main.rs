mod app;
mod bash;
mod config;
mod menu;
mod types;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use menu::Menu;
use std::io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to execute to get menu JSON
    #[arg(short, long)]
    command: Option<String>,
    
    /// JSON file to read menu from
    #[arg(short, long)]
    file: Option<String>,
    
    /// Width of the menu (default: 60% of terminal)
    #[arg(short = 'W', long)]
    width: Option<u16>,
    
    /// Height of the menu (default: based on content)
    #[arg(short = 'H', long)]
    height: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Get menu configuration
    let menu_json = if let Some(command) = args.command {
        bash::execute_menu_command(&command).await?
    } else if let Some(file) = args.file {
        std::fs::read_to_string(file)?
    } else {
        anyhow::bail!("Either --command or --file must be specified");
    };
    
    // Parse menu
    let mut menu = Menu::from_json(&menu_json)?;
    
    // Override dimensions if specified
    if let Some(width) = args.width {
        menu.config.width = width;
    } else if menu.config.width == 0 {
        menu.config.width = 120; // Default width
    }
    
    if let Some(height) = args.height {
        menu.config.height = height;
    } else if menu.config.height == 0 {
        menu.config.height = 20; // Default height
    }
    
    // Initialize terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    terminal.clear()?;
    
    // Run app
    let mut app = App::new(menu);
    let result = app.run(&mut terminal).await;
    
    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    // Handle result
    if let Ok(Some(selected)) = result {
        // Output the selected item as JSON
        let response = types::MenuResponse {
            selected: Some(selected.clone()),
            action: selected.action.unwrap_or_else(|| "select".to_string()),
        };
        
        println!("{}", serde_json::to_string(&response)?);
        
        // Execute command if present
        if let Some(command) = selected.command {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .spawn()?;
        }
    }
    
    Ok(())
}
