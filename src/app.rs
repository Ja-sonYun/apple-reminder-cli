use reminders_sc::shortcuts::fetch::fetch_todos;
use reminders_sc::todo::todo_obj::{Reminder, TodoObj};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState},
    Frame, Terminal,
};

use serde_yaml;

use crossterm::event::{self, Event, KeyCode};
use std::io;

pub enum TableChunk {
    TodoList,
    SelectedTodo,
}

enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    todo_list_state: TableState,
    selected_todo_state: TableState,
    highlighted: TableChunk,
    todos: Vec<TodoObj>,
    input: String,
    input_mode: InputMode,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            todo_list_state: TableState::default(),
            selected_todo_state: TableState::default(),
            highlighted: TableChunk::TodoList,
            todos: App::fetch_todos().expect("Failed to fetch todos"),
            input: String::new(),
            input_mode: InputMode::Normal,
        };
        app.todo_list_state.select(Some(0));
        app.selected_todo_state.select(Some(0));
        app
    }

    fn fetch_todos() -> Result<Vec<TodoObj>, serde_yaml::Error> {
        let fetched = fetch_todos().expect("Failed to fetch todos");
        let reminder: Reminder = serde_yaml::from_str(&fetched)?;

        Ok(reminder.todos)
    }

    pub fn next(&mut self) {
        let selected_table_prop = match self.highlighted {
            TableChunk::TodoList => (&mut self.todo_list_state, self.todos.len(), 1),
            TableChunk::SelectedTodo => (&mut self.selected_todo_state, TodoObj::item_count(), 2),
        };
        let i = match selected_table_prop.0.selected() {
            Some(i) => {
                if i >= selected_table_prop.1 - selected_table_prop.2 {
                    0
                } else {
                    i + selected_table_prop.2
                }
            }
            None => 0,
        };
        selected_table_prop.0.select(Some(i));
    }

    pub fn previous(&mut self) {
        let selected_table_prop = match self.highlighted {
            TableChunk::TodoList => (&mut self.todo_list_state, self.todos.len(), 1),
            TableChunk::SelectedTodo => (&mut self.selected_todo_state, TodoObj::item_count(), 2),
        };
        let i = match selected_table_prop.0.selected() {
            Some(i) => {
                if i == 0 {
                    selected_table_prop.1 - selected_table_prop.2
                } else {
                    i - selected_table_prop.2
                }
            }
            None => 0,
        };
        selected_table_prop.0.select(Some(i));
    }

    pub fn select(&mut self, target_chunk: TableChunk) {
        self.selected_todo_state.select(Some(0));
        self.highlighted = target_chunk
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match app.highlighted {
                    TableChunk::TodoList => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Enter | KeyCode::Char('l') => app.select(TableChunk::SelectedTodo),
                        _ => {}
                    },
                    TableChunk::SelectedTodo => match key.code {
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Char('q') | KeyCode::Char('h') => app.select(TableChunk::TodoList),
                        KeyCode::Enter | KeyCode::Char('l') => app.input_mode = InputMode::Editing,
                        _ => {}
                    },
                },
                InputMode::Editing => {
                    match key.code {
                        // KeyCode::Enter => {
                        //     app.messages.push(app.input.drain(..).collect());
                        // }
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
                    }
                }
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(20),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    render_todos(f, app, &rects[0]);
    render_selected_todo(f, app, &rects[1]);
    render_input_area(f, app, &rects[2]);
}

fn render_todos<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: &Rect) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::White);
    let headers_declared = Reminder::get_row_meta();
    let header_cells = headers_declared
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    let header = Row::new(header_cells).style(normal_style).height(1);
    let rows = app.todos.iter().map(TodoObj::get_cell);
    let reminder_headers_width = Reminder::get_header_width();
    let todo_table = Table::new(rows)
        .header(header)
        .block(Block::default())
        .highlight_style(selected_style)
        .widths(&reminder_headers_width);

    f.render_stateful_widget(todo_table, *rect, &mut app.todo_list_state);
}

fn render_selected_todo<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: &Rect) {
    let todo_rows = match app.todo_list_state.selected() {
        Some(i) => app.todos[i].show(),
        None => vec![],
    };
    let selected_style = match app.highlighted {
        TableChunk::SelectedTodo => Style::default().add_modifier(Modifier::REVERSED),
        _ => Style::default(),
    };
    let block = Block::default().borders(Borders::TOP);
    let reminder_headers_width = vec![Constraint::Percentage(100)];
    let selected_todo_table = Table::new(todo_rows)
        .block(block)
        .highlight_style(selected_style)
        .widths(&reminder_headers_width);

    f.render_stateful_widget(selected_todo_table, *rect, &mut app.selected_todo_state);
}

fn render_input_area<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: &Rect) {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Modify"));

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => f.set_cursor(rect.x + app.input.len() as u16 + 1, rect.y + 1),
    }

    f.render_widget(input, *rect);
}
