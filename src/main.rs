use journal_rs::prelude::*;

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
                println!("{}", submenu.name());
            });

            println!("\nExit");

            return true;
        }))
        .add_submenu("Create Journal", create_journal)
        .add_submenu("Open Journal", open_journal)
        .build();

    while !exit {
        exit = main_menu.run(Some("main"));
    }
}
