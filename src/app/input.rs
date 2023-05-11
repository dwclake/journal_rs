use std::io::*;

pub struct InputHandler<'a> {
    args: Vec<&'a str>,
    instruction: &'a str,
    prompt: &'a str,
    response: String
}

impl<'a> InputHandler<'a> {
    pub fn new(instruction: &'a str, prompt: &'a str, args: Vec<&'a str>)  -> InputHandler<'a> {
        InputHandler { 
            args,
            instruction,
            prompt,
            response: String::new() 
        }
    }

    pub fn call(&'a mut self) -> &'a str {
        self.get_input()
    }

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

    fn check_input(&self) -> bool {
        for &arg in &self.args {
            if arg == self.response {
                return true;
            } else if arg[0..1] == self.response[0..1] {
                return true;
            }
        }
        false
    }
}
