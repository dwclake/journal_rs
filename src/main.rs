struct Menu {
    fns: Vec<Box<dyn FnMut()>>,
}

fn main() {
    let message = "Hello, world!".to_string();
    let x = 5;
    let exit = false;

    let mut menu = Menu { fns: Vec::new() };

    menu.fns.push(Box::new(
        move || println!("{}", message)
    ));

    menu.fns.push(Box::new(
        move || println!("{}", x)
    ));

    menu.fns.push(Box::new(
        move || println!("{}", exit)
    ));

    for mut f in menu.fns {
        f();
    }
}
