use std::{io, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::models::Message;

pub fn handle_event() -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }

    Ok(None)
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('m') => Some(Message::Maintain),
        KeyCode::Char('f') => Some(Message::Fill),
        _ => None,
    }
}
