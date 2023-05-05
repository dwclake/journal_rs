use std::collections::HashMap;

pub struct Menu {
    name: String,
    fns: HashMap<String, Box<dyn Fn(&Menu) -> bool>>,
    submenus: HashMap<String, Menu>
}

impl Menu {
    
    pub fn builder() -> MenuBuilder {
        MenuBuilder {
            name: String::new(),
            fns: HashMap::new(),
            submenus: HashMap::new()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn call(& self, name: &str) -> bool {
        self.fns.get(name).expect("fn not found")(self)
    }

    pub fn sub(&self, name: &str) -> &Menu {
        self.submenus.get(name).expect("submenu not found")
    }

    pub fn for_each_sub<F>(&self, mut f: F) where F: FnMut(&Menu) {
        for (_, submenu) in &self.submenus {
            f(submenu);
        }
    }
}

pub struct MenuBuilder {
    pub name: String,
    pub fns: HashMap<String, Box<dyn Fn(&Menu) -> bool>>,
    pub submenus: HashMap<String, Menu>
}

impl MenuBuilder {

    pub fn name(mut self, name: &str) -> MenuBuilder {
        self.name = String::from(name);
        self
    }

    pub fn add_fn(mut self, name: &str, f: Box<dyn Fn(&Menu) -> bool>) -> MenuBuilder {
        self.fns.insert(String::from(name), f);
        self
    }

    pub fn add_submenu(mut self, name: &str, submenu: Menu) -> MenuBuilder {
        self.submenus.insert(String::from(name), submenu);
        self
    }

    pub fn build(self) -> Menu {
        Menu {
            name: self.name,
            fns: self.fns,
            submenus: self.submenus
        }
    }
}
