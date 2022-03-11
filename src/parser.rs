use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

use serde_json::{json, Result};

use crate::todo::Todo;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Fail"))
        .collect::<Vec<String>>())
}

pub fn write_json(todo: &mut Todo, filename: &str) {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(filename);
    let json = serde_json::to_string(todo).unwrap();
    file.unwrap().write_all(format!("{}\n", &json).as_bytes());
}

pub fn write_delete(todo: &mut Vec<Todo>, filename: &str) {
    fs::remove_file(filename);
    let mut count = 1;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(filename);
    let mut f = file.unwrap();
    for i in todo {
        i.number = count;
        let json = serde_json::to_string(&i).unwrap();
        f.write(format!("{}\n", &json).as_bytes());
        count += 1
    }
}

pub fn write_edit(
    todo: &mut Vec<Todo>,
    filename: &str,
    number: i32,
    new_title: &Option<String>,
    new_content: &Option<String>,
) {
    fs::remove_file(filename);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(filename);
    let mut f = file.unwrap();
    for i in todo {
        if i.number == number {
            match new_title {
                None => continue,
                Some(v) => i.title = v.to_string(),
            }
            match new_content {
                None => continue,
                Some(v) => i.content = v.to_string(),
            }
        }
        let json = serde_json::to_string(i).unwrap();
        f.write(format!("{}\n", &json).as_bytes());
    }
}

pub fn get_todo_vec(filename: &str) -> Result<Vec<Todo>> {
    let f = read_lines(filename).unwrap();
    let mut vec: Vec<Todo> = Vec::new();
    let num = f.iter().map(|x| x.to_string()).collect::<Vec<String>>();

    for mut i in num {
        if !i.is_empty() {
            vec.push(serde_json::from_str(&i.as_str()).unwrap());
        }
    }

    Ok(vec)
}

pub fn is_empty(vec_string: &Vec<String>) -> bool {
    vec_string.is_empty()
}

pub fn get_line(todo_num: i32, filename: &str) -> i32 {
    let todos = get_todo_vec(filename).unwrap();
    let mut count = 0;
    for i in todos {
        if i.number == todo_num {
            break;
        }
        count += 1
    }
    count
}
