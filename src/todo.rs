use std::collections::HashMap;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::parser::{get_todo_vec, is_empty, read_lines, write_delete, write_edit};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub number: i32,
    pub content: String,
    pub date: String,
}

pub enum Status {
    Success,
    Failed,
}

// pub fn show_todos(filename: &str) {
//     let todos = get_todo_vec(filename);
//     println!("---------------------TODOS----------------------");
//     for i in todos.unwrap() {
//         println!("[{}]------------[{}]-------------", i.number, i.date);
//         println!("{}", i.title);
//         println!("---------------------------------");
//         println!("{}", i.content);
//     }
// }

pub fn edit_todo(
    filename: &str,
    number: i32,
    new_content: &Option<String>,
) -> Status {
    let mut success = false;
    if let Ok(f) = read_lines(filename) {
        let file = read_lines(filename).unwrap();
        if !is_empty(&file) {
            let mut todos = get_todo_vec(filename).unwrap();

            if let Some(_) = todos.get(number as usize - 1) {
                success = true;
                write_edit(&mut todos, filename, number, new_content);
            } else {
                success = false;
            }
        }
    }
    if success {
        Status::Success
    } else {
        Status::Failed
    }
}

pub fn delete_todo(todo_num: usize, filename: &str) -> Status {
    let mut success = false;
    if let Ok(f) = read_lines(filename) {
        let file = read_lines(filename).unwrap();
        if !is_empty(&file) {
            let mut todos = get_todo_vec(filename).unwrap();

            if let Some(_) = todos.get(todo_num - 1) {
                success = true;
                &mut todos.remove(todo_num - 1);
                write_delete(&mut todos, filename);
            } else {
                success = false;
            }
        } else {}
    }
    if success {
        Status::Success
    } else {
        Status::Failed
    }
}
