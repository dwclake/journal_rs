pub mod page;

use crate::prelude::Page;

pub struct Journal<'a> {
    name: &'a str,
    pages: Vec<Page<'a>>,
    size: u16
}

impl<'a> Journal<'a> {

}
