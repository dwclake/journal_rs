use std::collections::HashMap;
use std::mem::take;

/// Struture representing a menu
pub struct Menu {
    name: String,
    fns: HashMap<String, Box<dyn Fn(&Menu) -> bool>>,
    submenus: HashMap<String, Menu>
}

impl Menu {
    /// Returns a pointer to a new MenuBuiler instance
    pub fn builder() -> Box<MenuBuilder> {
        Box::new(MenuBuilder {
            name: String::new(),
            fns: HashMap::new(),
            submenus: HashMap::new()
        })
    }

    /// Return the name of the menu
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Calls the specified function stored in the menu
    pub fn call(& self, name: &str) -> bool {
        self.fns.get(name).expect("fn not found")(self)
    }

    /// Calls the specified submenu stored in the menu
    pub fn sub(&self, name: &str) -> &Menu {
        self.submenus.get(name).expect("submenu not found")
    }

    /// Applies a callback function to each submenu stored in the menu
    pub fn for_each_sub<F>(&self, mut f: F) where F: FnMut(&Menu) {
        for (_, submenu) in &self.submenus {
            f(submenu);
        }
    }
}

/// Builder structure for creating a new menu
pub struct MenuBuilder {
    name: String,
    fns: HashMap<String, Box<dyn Fn(&Menu) -> bool>>,
    submenus: HashMap<String, Menu>
}

impl MenuBuilder {
    /// Sets the name of the menu being built
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    /// Adds a function to the menu being built
    pub fn add_fn(&mut self, name: &str, f: Box<dyn Fn(&Menu) -> bool>) -> &mut Self {
        self.fns.insert(String::from(name), f);
        self
    }

    /// Adds a submenu to the menu being built
    pub fn add_submenu(&mut self, name: &str, submenu: Menu) -> &mut Self {
        self.submenus.insert(String::from(name), submenu);
        self
    }

    /// Takes the menu builder and moves its attributes into a new Menu instance and returns it
    pub fn build(&mut self) -> Menu {
        Menu {
            name: take(&mut self.name),
            fns: take(&mut self.fns),
            submenus: take(&mut self.submenus)
        }
    }
}
