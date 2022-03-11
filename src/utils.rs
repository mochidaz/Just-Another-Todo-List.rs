use core::result::Result::Ok;
use std::fs;

use crate::parser::{get_todo_vec, is_empty, read_lines, write_delete, write_edit};
use crate::todo::{Status, Todo};

pub fn generate_number(todo: &mut Todo, filename: &str) {
    if let Ok(f) = read_lines(filename) {
        if !is_empty(&f) {
            let todo_vec = get_todo_vec(filename).unwrap();
            let max = todo_vec.iter().map(|x| x.number).max();
            todo.number = match max {
                Some(n) => n + 1,
                None => 1,
            }
        }
    } else {
        fs::File::create(filename);
    }
}
