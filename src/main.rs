use std::{error::Error, io};
use std::time::SystemTime;

use chrono::prelude::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    Terminal,
    text::{Span, Spans, Text}, widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tui::layout::Alignment;
use tui::style::Color::Rgb;
use tui::widgets::BorderType;
use unicode_width::UnicodeWidthStr;

use todo::{delete_todo, edit_todo};
use utils::generate_number;

use crate::parser::{get_line, get_todo_vec, write_json};
use crate::todo::{Status, Todo};

mod parser;
mod todo;
mod utils;

enum InputMode {
    Normal,
    Editing,
    Delete,
}

struct App {
    input: String,
    input_mode: InputMode,
    todos: Vec<Todo>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            todos: get_todo_vec(FILENAME).unwrap(),
        }
    }
}

const FILENAME: &str = "todo.txt";

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let system_time = SystemTime::now();
        let time: DateTime<Local> = system_time.into();

        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('d') => {
                        app.input_mode = InputMode::Delete;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let mut todo = Todo {
                            number: 1,
                            content: app.input.drain(..).collect(),
                            date: time.format("%d/%m/%Y %T").to_string(),
                        };

                        app.todos.push(todo.clone());
                        generate_number(&mut todo, FILENAME);
                        write_json(&mut todo, FILENAME);
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::Delete => match key.code {
                    KeyCode::Enter => {
                        let num: String = app.input.drain(..).collect();
                        delete_todo(num.parse().unwrap(), FILENAME);
                        app.todos = get_todo_vec(FILENAME).unwrap();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start inserting your todos, "),
                Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start deleting your todos."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add your new todo"),
            ],
            Style::default(),
        ),
        InputMode::Delete => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" your todo number to delete it"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Delete => Style::default().fg(Rgb(255, 0, 0))
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            {}

        InputMode::Editing => {
            f.set_cursor(
                chunks[1].x + app.input.width() as u16 + 1,
                chunks[1].y + 1,
            )
        }
        InputMode::Delete => {
            f.set_cursor(
                chunks[1].x + app.input.width() as u16 + 1,
                chunks[1].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{} | {} : {}", (i + 1), m.date, m.content)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Todo List"));
    f.render_widget(messages, chunks[2]);
}