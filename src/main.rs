use chrono::prelude::*;
use std::time::SystemTime;
use todo::{delete_todo, edit_todo};

use utils::generate_number;

use crate::parser::{get_line, write_json};
use crate::todo::{show_todos, Status, Todo};

mod parser;
mod todo;
mod utils;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tui::layout::Alignment;
use tui::widgets::BorderType;
use unicode_width::UnicodeWidthStr;

struct App {

}

fn main() {

}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title("Test")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Plain);

    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(5)
        .constraints(C)
}