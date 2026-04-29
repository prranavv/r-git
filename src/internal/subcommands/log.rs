use std::io::{self};
use std::io::Write;
use crate::{Result, internal::utils::{parse_commit_history}};
use crossterm::{event::{self, Event, KeyCode}};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn log()->Result<()>{
    let result  = parse_commit_history()?;
    let mut stdout = io::stdout();
    writeln!(stdout,"{}",result.trim())?;
    println!("\nPress any key (q to exit)...");
    
    // Enable raw mode to read keypresses immediately
    enable_raw_mode()?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restore terminal to default mode
    disable_raw_mode()?;
    Ok(())
}