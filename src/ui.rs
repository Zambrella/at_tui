use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, Paragraph},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::app::{App, Sections};

const PRIMARY_COLOR: Color = Color::Rgb(255, 105, 0);
const UNSELECTED_COLOR: Color = Color::Rgb(128, 128, 128);

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[1]);

    // Hotkeys
    frame.render_widget(
        Paragraph::new("q (Quit) | Tab (Switch Section) | Up/Down (Navigate)")
            .block(Block::new().borders(Borders::ALL).title_top("Actions")),
        layout[0],
    );

    // Main sections
    // Files
    frame.render_widget(
        List::new(
            app.at_sign_files
                .files
                .iter()
                .enumerate()
                .map(|(index, file)| {
                    Text::styled(
                        file.file_name().unwrap().to_str().unwrap(),
                        if index == app.at_sign_files.selected_index {
                            Style::new().bg(PRIMARY_COLOR)
                        } else {
                            Style::default()
                        },
                    )
                }),
        )
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(match app.selected_section {
                    Sections::Files => Style::default().fg(PRIMARY_COLOR),
                    _ => Style::default().fg(UNSELECTED_COLOR),
                })
                .title_top("AtSigns"),
        ),
        content_layout[0],
    );

    // Logs

    frame.render_widget(LogWidget { app }, content_layout[1]);
}

struct LogWidget<'a> {
    app: &'a mut App,
}

impl<'a> Widget for LogWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiLoggerWidget::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(match self.app.selected_section {
                        Sections::Logs => Style::default().fg(PRIMARY_COLOR),
                        _ => Style::default().fg(UNSELECTED_COLOR),
                    })
                    .title_top("Log"),
            )
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default())
            .render(area, buf);
    }
}
