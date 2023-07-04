use std::io::*;

/// Structure use to handle user input
///
/// # Example
/// ```
/// use review::prelude::*;
///
/// let args = vec![
///     "a", "b", "c"
/// ];
///
/// let mut ih = InputHandler::new(
///     "Choose a menu",
///     "$",
///     args
/// );
///
/// //let user_response: &str = ih.call();
/// ```
pub struct InputHandler<'a> {
    args: Vec<&'a str>,
    instruction: &'a str,
    prompt: &'a str,
    response: String
}

impl<'a> InputHandler<'a> {
    /// Return a new InputHandler structure
    pub fn new(instruction: &'a str, prompt: &'a str, args: Vec<&'a str>)  -> InputHandler<'a> {
        InputHandler { 
            args,
            instruction,
            prompt,
            response: String::new() 
        }
    }

    /// Runs the input handler, returning a string slice
    pub fn call(&'a mut self) -> &'a str {
        self.get_input()
    }

    /// Gets input from the user, looping until the user enters a acceptable response
    fn get_input(&'a mut self) -> &'a str {
        print!("{}\n{} ", self.instruction, self.prompt);
        stdout().flush().unwrap();

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
                    print!("Input not recognized, please try again\n{} ", self.prompt);
                    stdout().flush().unwrap();
                },
                false => {
                    return &self.response[..];
                }
            }
        }
    }

    /// Checks if user input matches any of the arguments
    fn check_input(&self) -> bool {
        for &arg in &self.args {
            if arg == self.response {
                return true;
            }
        }
        false
    }
}
