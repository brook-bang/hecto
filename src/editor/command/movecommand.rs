use crossterm::event::{
    Event, KeyCode::{
        self,Backspace, Char, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab, Up,
    }, KeyEvent, KeyModifiers,
};

use super::size::Size;

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageDown => Ok(Self::PageDown),
                PageUp => Ok(Self::PageUp),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}

#[derive(Clone,Copy)]
pub enum Edit {
    Insert(char),
    InsertNewline,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code,event.modifiers) {
            (Char(character),KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab,KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter,KeyModifiers::NONE) => Ok(Self::InsertNewline),
            (Backspace,KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code,event.modifiers
            )),
        }
    }
}