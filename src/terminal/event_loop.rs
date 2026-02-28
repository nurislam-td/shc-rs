use color_eyre::eyre::Result;
use ratatui::{
    crossterm::{
        event,
        event::{Event, KeyCode},
    }, widgets::{Paragraph, Widget},
    DefaultTerminal,
    Frame,
};

#[derive(Debug, Default)]
struct SshConnection {
    host: String,
    user: String,
    port: i32,
    name: String,
}

#[derive(Debug, Default)]
struct AppState {
    connections: Vec<SshConnection>,
}

fn render(frame: &mut Frame, app_state: &AppState) {
    Paragraph::new("Hello, world!").render(frame.area(), frame.buffer_mut())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = AppState::default();
    loop {
        terminal.draw(|frame| render(frame, &state))?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
