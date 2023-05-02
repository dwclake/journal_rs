

pub struct Menu {
    pub fns: Vec<Box<dyn FnMut() -> bool>>,
    pub submenus: Vec<Menu>
}