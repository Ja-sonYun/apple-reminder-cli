use crate::todo::deserialize::{DueDate, IsCompleted};
use itertools::Itertools;
use serde::Deserialize;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState},
    Frame, Terminal,
};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Reminder {
    pub todos: Vec<TodoObj>,
}

impl Reminder {
    pub fn get_row_meta() -> Vec<&'static str> {
        vec!["", "list", "description", "tags"]
    }

    pub fn get_header_width() -> Vec<Constraint> {
        vec![
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Percentage(80),
            Constraint::Min(25),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TodoObj {
    pub title: String,
    is_completed: IsCompleted,
    due_date: DueDate,
    list: String,
    tags: Vec<String>,
    note: String,
}

impl TodoObj {
    fn get_row(&self) -> Vec<String> {
        vec![
            self.is_completed.as_icon(),
            self.list.clone(),
            self.title.clone(),
            self.format_tags(),
        ]
    }

    fn get_height(&self) -> u16 {
        (self.note.chars().filter(|c| *c == '\n').count() + 1) as u16
    }

    fn format_tags(&self) -> String {
        self.tags
            .iter()
            .map(|t| "#".to_owned() + t)
            .intersperse(", ".to_string())
            .collect()
    }

    pub fn show(&self) -> Vec<Row> {
        let header_style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        vec![
            Row::new(vec!["[Title]".to_string()]).style(header_style),
            Row::new(vec![self.title.clone()]),
            Row::new(vec!["[List]".to_string()]).style(header_style),
            Row::new(vec![self.list.clone()]),
            Row::new(vec!["[Status]".to_string()]).style(header_style),
            Row::new(vec![self.is_completed.as_string()]),
            Row::new(vec!["[Tags]".to_string()]).style(header_style),
            Row::new(vec![self.format_tags()]),
            Row::new(vec!["[Note]".to_string()]).style(header_style),
            Row::new(vec![self.note.clone()]).height(self.get_height()),
        ]
    }
}

impl TodoObj {
    pub fn new_empty() -> TodoObj {
        TodoObj {
            title: String::new(),
            is_completed: IsCompleted::NO,
            due_date: DueDate::None,
            list: String::new(),
            tags: Vec::new(),
            note: String::new(),
        }
    }

    pub fn get_cell(todo: &TodoObj) -> Row {
        let height = 1;
        let cells = todo.get_row();
        let line_style = match &todo.is_completed {
            IsCompleted::YES => Style::default().add_modifier(Modifier::DIM),
            IsCompleted::NO => Style::default(),
        };
        Row::new(cells).height(height as u16).style(line_style)
    }

    pub fn item_count() -> usize {
        10 // 5 * 2 for the header
    }
}
