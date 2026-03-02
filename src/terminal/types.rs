use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget};

pub struct SshConnection {
    host: String,
    user: String,
    port: i32,
    name: String,
}

pub enum States {
    Start,
    Running,
    End,
}

pub enum Screen {
    Main,
    Editor,
    Exit,
}

pub enum OnEdit {
    Host,
    User,
    Port,
    Name,
}

pub struct AppState {
    pub connections: Vec<SshConnection>,
    pub current_state: States,
    pub input: String,
    pub currently_editing: Option<OnEdit>,
    pub currently_row: usize,
    pub current_screen: Screen,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            connections: Vec::default(),
            current_state: States::Start,
            input: String::default(),
            currently_editing: Option::default(),
            current_screen: Screen::Main,
            currently_row: 0,
        }
    }
}

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            Screen::Main => {}
            Screen::Editor => {}
            Screen::Exit => {}
        }
    }
}

impl AppState {
    pub fn add_connection(&mut self, connection: SshConnection) {
        self.connections.push(connection);
    }
}
