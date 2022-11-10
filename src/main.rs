use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod editor;
use editor::Editor;

fn to_ctrl_byte(c: char) -> char {
    let byte = c as u8;
    // byte & 0b0001_1111
    c
}

fn main() {
    let editor = Editor::default();
    editor.run();
}
