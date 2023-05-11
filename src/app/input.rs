use std::io::*;

pub struct InputHandler<'a> {
    args: Vec<&'a str>,
    response: String
}

impl<'a> InputHandler<'a> {
    pub fn new(args: Vec<&str>) -> InputHandler {
        InputHandler { args, response: String::new() }
    }

    pub fn call(&'a mut self) -> &'a str {
        self.get_input()
    }

    fn get_input(&'a mut self) -> &'a str {
        loop {
            self.response = String::new();
            stdin().read_line(&mut self.response)
                .unwrap();

            self.response = self.response
                .to_lowercase()
                .trim()
                .to_owned();

            match !self.check_input() {
                true => {
                    print!("Input not recognized, please try again\n> ");
                    stdout().flush().unwrap();
                },
                false => {
                    return &self.response[..];
                }
            }
        }
    }

    fn check_input(&self) -> bool {
        for &arg in &self.args {
            if arg == self.response {
                return true;
            }
        }
        false
    }
}
