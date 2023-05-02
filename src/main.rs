use journal_rs::prelude::*;

fn main() {
    let mut exit = false;

    let mut menu = app::Menu { 
        fns: Vec::new(),
        submenus: Vec::new() 
    };

    menu.fns.push(Box::new(move || {
        println!("{}", "Hello World!");
        return true;
    }));

    while !exit {
        exit = menu.fns[0]();
    }
}
