use journal_rs::prelude::*;
use colored::Colorize;

fn main() {
    let mut exit = false;

    let create_journal = menu::Menu::builder()
        .name("Create Journal")
        .build();

    let open_journal = menu::Menu::builder()
        .name("Open Journal")
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

            menu.run(Some("input"))            
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
                "c" => menu.run(Some("Create Journal")),
                "o" => menu.run(Some("Open Journal")),
                _ => {
                    return false;
                }
            }
        }))
        .add_submenu("Create Journal", create_journal)
        .add_submenu("Open Journal", open_journal)
        .build();

    while !exit {
        exit = main_menu.run(Some("main"));
    }
}
