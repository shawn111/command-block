use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, stdout};
use tokio::process::Command;
use std::process::Stdio;



pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub struct ShellApp<F>
where
    F: Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send>>
        + Send
        + Sync
        + 'static,
{
    input: String,
    blocks: Vec<(String, String)>,
    run_cmd: F,
}

impl<F> ShellApp<F>
where
    F: Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send>>
        + Send
        + Sync
        + 'static,
{
    pub fn new(runner: F) -> Self {
        Self {
            input: String::new(),
            blocks: vec![],
            run_cmd: runner,
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        let mut terminal = setup_terminal()?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [Constraint::Min(1), Constraint::Length(3)].as_ref(),
                    )
                    .split(f.size());

                let text = self.blocks.iter()
                    .map(|(cmd, out)| format!("$ {}\n{}", cmd, out))
                    .collect::<Vec<_>>()
                    .join("\n\n");

                let history = Paragraph::new(text)
                    .block(Block::default().title("Shell Output").borders(Borders::ALL));
                f.render_widget(history, chunks[0]);

                let input = Paragraph::new(self.input.as_str())
                    .block(Block::default().title("Command").borders(Borders::ALL).style(Style::default()));
                f.render_widget(input, chunks[1]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => self.input.push(c),
                        KeyCode::Backspace => { self.input.pop(); }
                        KeyCode::Enter => {
                            let cmd = self.input.trim().to_string();
                            self.input.clear();

                            if cmd.eq_ignore_ascii_case("/exit") || cmd.eq_ignore_ascii_case("/quit") {
                                break;
                            }

                            let output = (self.run_cmd)(&cmd).await;
                            self.blocks.push((cmd, output));
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        restore_terminal(terminal)?;
        Ok(())
    }
}

