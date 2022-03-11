use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use std::time::SystemTime;
use todo::{delete_todo, edit_todo};

use utils::generate_number;

use crate::parser::{get_line, write_json};
use crate::todo::{show_todos, Status, Todo};

mod parser;
mod todo;
mod utils;

fn args() -> ArgMatches<'static> {
    App::new("Todo")
        .version("0.1.0")
        .author("Rahman Hakim <rahmanhakim2435@protonmail.com")
        .about("Todo List")
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .takes_value(true)
                .help("Title of the todo")
                .required(false),
        )
        .arg(
            Arg::with_name("content")
                .short("c")
                .long("content")
                .takes_value(true)
                .help("Content of the Todo")
                .required(false),
        )
        .arg(
            Arg::with_name("show")
                .short("s")
                .long("show")
                .takes_value(false)
                .help("Show todos")
                .required(false),
        )
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .takes_value(true)
                .help("Edit todo")
                .required(false),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .takes_value(true)
                .help("Delete a todo. todo -d <todo number>")
                .required(false),
        )
        .get_matches()
}

fn main() {
    let number = 1;
    let filename = "todo.txt";
    let system_time = SystemTime::now();
    let time: DateTime<Local> = system_time.into();
    let arguments = args();
    let show_exists = arguments.is_present("show");
    let content_exists = arguments.is_present("content");
    let delete_exists = arguments.is_present("delete");
    let title_exists = arguments.is_present("title");
    let edit_exists = arguments.is_present("edit");
    let edit = arguments.value_of("edit").unwrap_or("0").to_string();
    let content = arguments.value_of("content").unwrap_or("New").to_string();
    let delete = arguments.value_of("delete").unwrap_or("Default");
    let title = arguments
        .value_of("title")
        .unwrap_or("New Project")
        .to_string();

    if !edit_exists {
        match (title_exists, content_exists, delete_exists) {
            (true, true, _) => {
                let mut t = Todo {
                    number,
                    title: title.clone(),
                    content: content.clone(),
                    date: time.format("%d/%m/%Y %T").to_string(),
                };
                generate_number(&mut t, filename);
                write_json(&mut t, filename);
            }
            (false, true, _) => {
                let mut t = Todo {
                    number,
                    title: "Untitled Todo".to_string(),
                    content: content.clone(),
                    date: time.format("%d/%m/%Y %T").to_string(),
                };
                generate_number(&mut t, filename);
                write_json(&mut t, filename);
            }
            (true, false, _) => {
                let mut t = Todo {
                    number,
                    title: title.clone(),
                    content: "...".to_string(),
                    date: time.format("%d/%m/%Y %T").to_string(),
                };
                generate_number(&mut t, filename);
                write_json(&mut t, filename);
            }
            (false, false, true) => {}
            (false, false, false) => {
                if !show_exists {
                    println!("Why do you even use this, captain?")
                }
            }
        }
    } else {
        match (title_exists, content_exists) {
            (true, true) => {
                edit_todo(
                    filename,
                    edit.parse::<i32>().unwrap(),
                    &Some(title.to_string()),
                    &Some(content.to_string()),
                );
            }
            (true, false) => {
                edit_todo(
                    filename,
                    edit.parse::<i32>().unwrap(),
                    &Some(title.to_string()),
                    &None,
                );
            }
            (false, true) => {
                edit_todo(
                    filename,
                    edit.parse::<i32>().unwrap(),
                    &None,
                    &Some(content.to_string()),
                );
            }
            (false, false) => {
                println!("Whatchu wanna edit lol?")
            }
        }
    }

    if show_exists {
        show_todos(filename);
    }

    if delete_exists {
        let del = delete_todo(delete.parse::<usize>().unwrap(), filename);
        match del {
            Status::Success => {
                println!("Successfully deleted");
            }
            Status::Failed => {
                println!("The todo with following number does not exist!")
            }
        };
    }
}
