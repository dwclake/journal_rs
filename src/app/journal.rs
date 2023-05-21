pub mod page;

use crate::prelude::Page;

pub struct Journal<'a> {
    name: &'a str,
    pages: Vec<Page>,
    size: u16
}

impl<'a> Journal<'a> {
    pub fn new(name: &'a str, size: u16) -> Journal<'a> {
        Journal {
            name,
            pages: Vec::new(),
            size
        }
    }
}
