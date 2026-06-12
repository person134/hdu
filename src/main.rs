#![allow(dead_code)]

mod action;
mod backend;
mod config;
mod scanner;
mod system;
mod ui;

use std::io;

use clap::Parser;
use crossterm::{
    execute,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use action::Commands;
use backend::Platform;
use config::Config;
use ui::AppUi;

fn main() -> io::Result<()> {
    let cli = action::Cli::parse();
    let platform = backend::detect_platform();
    let config = Config::load();

    match cli.command {
        Some(Commands::Scan { path }) => {
            match scanner::scan(&path) {
                Ok(root) => ui::print_scan(&root),
                Err(e) => eprintln!("Error scanning {}: {}", path.display(), e),
            }
        }
        Some(Commands::Tree { path }) => {
            match scanner::scan(&path) {
                Ok(root) => ui::print_tree(&root, 0),
                Err(e) => eprintln!("Error scanning {}: {}", path.display(), e),
            }
        }
        Some(Commands::Watch { refresh_rate, path }) => {
            ui::watch_disk_usage(&path, refresh_rate)?;
        }
        None => {
            run_tui(platform, &config, cli.refresh_rate)?;
        }
    }

    Ok(())
}

fn run_tui(platform: Platform, config: &Config, refresh_rate: u64) -> io::Result<()> {
    let (w, h) = size().unwrap_or((0, 0));
    if w < 50 || h < 10 {
        eprintln!("Terminal window is too small!");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppUi::new(platform, config, refresh_rate);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
