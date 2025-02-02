use crate::app::{App, AppResult, Sections};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::{info, log};
use tui_logger::TuiWidgetEvent;

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
        let state = &mut app.log_state;
        match key_event.code {
            KeyCode::Tab => {
                app.next_section();
            }
            KeyCode::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
            KeyCode::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
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
                info!("Selecting previous file.");
                app.at_sign_files.select_previous();
            }
            KeyCode::Down => {
                info!("Selecting next file.");
                app.at_sign_files.select_next();
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
}
