use std::process::Command;
use std::io::*;

use colored::Colorize;

use journal_rs::prelude::*;
use journal_rs::prelude::input::InputHandler;

#[allow(unused_variables)]

fn main() {
    let mut exit = false;
    let create_journal = Menu::builder()
        .name("Create Journal")
        .add_fn("main", Box::new(|menu: &Menu| {
            Command::new("clear").status().expect("Failed to clear screen");
            menu.call("input")
        }))
        .add_fn("input", Box::new(|menu: &Menu| {
            false  
        }))
        .build();

    let open_journal = Menu::builder()
        .name("Open Journal")
        .add_fn("main", Box::new(|menu: &Menu| {
            Command::new("clear").status().expect("Failed to clear screen");
            menu.call("input")
        }))
        .add_fn("input", Box::new(|menu: &Menu| {
            false  
        }))
        .build();

    let main_menu = Menu::builder()
        .name("\t\t\t -- Main Menu --")
        .add_fn("main", Box::new(|menu: &Menu| {
            Command::new("clear").status().expect("Failed to clear screen");
            println!("{}", menu.name());

            menu.for_each_sub(|submenu| {
                let name = submenu.name().to_string();
                println!("{}{}", name[0..1].yellow(), &name[1..]);
            });

            println!("\n{}xit", "E".yellow());

            menu.call("input")            
        }))
        .add_fn("input", Box::new(|menu: &Menu| {

            print!("Select an option\n> ");
            stdout().flush().unwrap();
            
            let mut input = InputHandler::new(vec![
                "e", "c", "o",
                "exit", "create", "open"
            ]);

            return match input.call() {
                "e" | "exit" => {
                    println!("Exiting...");
                    true
                },
                "c" | "create" => menu.sub("create").call("main"),
                "o" | "open" => menu.sub("open").call("main"),
                _ => {
                    false
                }
            }
        }))
        .add_submenu("create", create_journal)
        .add_submenu("open", open_journal)
        .build();

    while !exit {
        exit = main_menu.call("main");
    }
}
