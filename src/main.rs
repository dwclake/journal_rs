use journal_rs::prelude::*;
use colored::Colorize;

fn main() {
    let mut exit = false;

    let create_journal = menu::Menu::builder()
        .name("Create Journal")
        .add_fn("main", Box::new(|menu: &menu::Menu| {
            false           
        }))
        .build();

    let open_journal = menu::Menu::builder()
        .name("Open Journal")
        .add_fn("main", Box::new(|menu: &menu::Menu| {
            false           
        }))
        .build();

    let main_menu = menu::Menu::builder()
        .name("\t\t\t -- Main Menu --")
        .add_fn("main", Box::new(|menu: &menu::Menu| {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", menu.name());

            menu.for_each_submenu(|submenu| {
                let name = submenu.name().to_string();
                println!("{}{}", name[0..1].yellow(), &name[1..]);
            });

            println!("\n{}xit", "E".yellow());

            menu.run("input")            
        }))
        .add_fn("input", Box::new(|menu: &menu::Menu| {
            println!("Select an option");  
            
            let mut input = String::new();

            std::io::stdin().read_line(&mut input).unwrap();

            match input[0..1].to_lowercase().as_str() {
                "e" => {
                    println!("Exiting...");
                    return true;
                },
                "c" => menu.menu("create").run("main"),
                "o" => menu.menu("open").run("main"),
                _ => {
                    return false
                }
            }
        }))
        .add_submenu("create", create_journal)
        .add_submenu("open", open_journal)
        .build();

    while !exit {
        exit = main_menu.run("main");
    }
}
