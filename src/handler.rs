use crate::app::{App, AppResult, Sections};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.selected_section {
        // Files section handlers
        Sections::Files => handle_files_section(key_event, app),
        // Logs section handlers
        Sections::Logs => handle_logs_section(key_event, app),
    }
    Ok(())
}

fn should_quit(key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => true,
        KeyCode::Char('c') | KeyCode::Char('C') => key_event.modifiers == KeyModifiers::CONTROL,
        _ => false,
    }
}

fn handle_logs_section(key_event: KeyEvent, app: &mut App) {
    if should_quit(&key_event) {
        app.quit();
    } else {
        match key_event.code {
            KeyCode::Tab => {
                app.next_section();
            }
            KeyCode::Up => {
                app.at_sign_files.select_previous();
            }
            KeyCode::Down => {
                app.at_sign_files.select_next();
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
}

fn handle_files_section(key_event: KeyEvent, app: &mut App) {
    if should_quit(&key_event) {
        app.quit();
    } else {
        match key_event.code {
            KeyCode::Tab => {
                app.next_section();
            }
            KeyCode::Up => {
                app.at_sign_files.select_previous();
            }
            KeyCode::Down => {
                app.at_sign_files.select_next();
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
}
