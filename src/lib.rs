use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::collections::BTreeMap;
use zellij_tile::prelude::{Event, register_plugin, BareKey};
use zellij_tile::{ZellijPlugin};
use zellij_tile::shim::report_panic;

#[derive(Default)]
struct State {
    input: String,
    blocks: Vec<(String, String)>,
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // No special configuration needed for now
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(key_event) => {
                match key_event.bare_key {
                    BareKey::Char(c) => {
                        self.input.push(c);
                        should_render = true;
                    }
                    BareKey::Backspace => {
                        self.input.pop();
                        should_render = true;
                    }
                    BareKey::Enter => {
                        let cmd = self.input.trim().to_string();
                        self.input.clear();
                        if cmd.eq_ignore_ascii_case("/exit") || cmd.eq_ignore_ascii_case("/quit") {
                            // In a plugin, we don't exit the whole app, but maybe clear state or show a message
                            self.blocks.push(("System".to_string(), "Exiting command-block plugin.".to_string()));
                        } else {
                            // For now, just echo the command
                            self.blocks.push((cmd.clone(), format!("Command: {}", cmd)));
                        }
                        should_render = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout())).expect("Failed to create terminal");
        let _ = terminal.draw(|f| {
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(
                    [ratatui::layout::Constraint::Min(1), ratatui::layout::Constraint::Length(3)].as_ref(),
                )
                .split(f.size());

            let text = self.blocks.iter()
                .map(|(cmd, out)| format!("$ {}\n{}", cmd, out))
                .collect::<Vec<_>>()
                .join("\n\n");

            let history = ratatui::widgets::Paragraph::new(text)
                .block(ratatui::widgets::Block::default().title("Shell Output").borders(ratatui::widgets::Borders::ALL));
            f.render_widget(history, chunks[0]);

            let input = ratatui::widgets::Paragraph::new(self.input.as_str())
                .block(ratatui::widgets::Block::default().title("Command").borders(ratatui::widgets::Borders::ALL).style(ratatui::style::Style::default()));
            f.render_widget(input, chunks[1]);
        });
    }
}

// This is the entry point for the Zellij plugin
register_plugin!(State);
