use std::env;
use std::path::PathBuf;
use std::{error, path::Path};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Sections {
    Files,
    Logs,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// User's at sign files.
    pub at_sign_files: FileSelector,
    /// The selected section.
    pub selected_section: Sections,
    /// Whether to show logs.
    pub show_logs: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            at_sign_files: FileSelector::default(),
            selected_section: Sections::Files,
            show_logs: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn next_section(&mut self) {
        self.selected_section = match self.selected_section {
            Sections::Files => Sections::Logs,
            Sections::Logs => Sections::Files,
        };
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[derive(Debug)]
pub struct FileSelector {
    pub files: Vec<PathBuf>,
    pub selected_index: usize,
}

impl Default for FileSelector {
    fn default() -> Self {
        Self {
            files: vec![],
            selected_index: 0,
        }
    }
}

impl FileSelector {
    /// Get all the user's at sign files.
    pub async fn get_at_sign_files(&mut self) -> AppResult<()> {
        let home = env::var("HOME").expect("HOME not set");
        let path = Path::new(&home).join(".atsign/keys/");
        let mut files = vec![];
        let mut entries = tokio::fs::read_dir(path)
            .await
            .map_err(|e| format!("Unable to read at_sign secrets: {}", e))?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        self.files = files;
        Ok(())
    }

    pub fn selected_file(&self) -> Option<&PathBuf> {
        self.files.get(self.selected_index)
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.files.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
