use crate::terminal::types::{AppState, Screen, States};
use color_eyre::eyre::Result;
use ratatui::{
    crossterm::{
        event,
        event::{Event, KeyCode, KeyEvent},
    }, widgets::Paragraph,
    DefaultTerminal,
    Frame,
};

fn render(app_state: &AppState, frame: &mut Frame) {
    // Paragraph::new("Hello, world!").render(frame.area(), frame.buffer_mut());
    frame.render_widget(Paragraph::new("Hello, world!"), frame.area());
}

fn handle_key_input(app_state: &mut AppState, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Esc => match app_state.current_screen {
            Screen::Main => app_state.current_state = States::End,
            Screen::Editor => app_state.current_screen = Screen::Main,
            Screen::Exit => app_state.current_state = States::End,
        },
        KeyCode::Char('j') => {
            app_state.currently_row = app_state.currently_row + 1 % app_state.connections.len();
        }
        KeyCode::Char('k') => {
            let conn_len = app_state.connections.len();
            app_state.currently_row = (app_state.currently_row + conn_len - 1) % conn_len;
        }
        KeyCode::Char('e') => {
            app_state.current_screen = Screen::Editor;
        }
        KeyCode::Char('q') => {
            app_state.current_screen = Screen::Exit;
            app_state.current_state = States::End;
        }
        _ => {}
    }

    Ok(())
}

fn handle_event(app_state: &mut AppState) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) => {
            handle_key_input(app_state, key_event)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = AppState::default();

    loop {
        match state.current_state {
            States::Start => state.current_state = States::Running,
            States::Running => {
                terminal.draw((|frame| render(&state, frame)))?;
                handle_event(&mut state);
            }
            States::End => {
                break;
            }
        }
    }

    Ok(())
}
