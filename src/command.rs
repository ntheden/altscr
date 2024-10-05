//use strprox::Autocompleter;
use crate::screen::Screen;
use casey::lower;
use std::str::FromStr;

// modified macro from
// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
            $($variant),*
        }
        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => lower!(stringify!($variant))),*
                }
            }
        }
    };
}

enum_str! {
    enum Commands {
        Clear,
        Exit,
        Help,
        Menu,
        Quit,
    }
}

impl FromStr for Commands {
    type Err = ();
    fn from_str(input: &str) -> Result<Commands, Self::Err> {
        let mut args: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        match args.pop() {
            Some(command) => {
                match command
                .to_lowercase()
                .as_str() {
                    "clear" => Ok(Commands::Clear),
                    "exit" => Ok(Commands::Exit),
                    "help" => Ok(Commands::Help),
                    "menu" => Ok(Commands::Menu),
                    "quit" => Ok(Commands::Quit),
                    _ => Err(()),
                }
            }
            _ => Err(())
        }
    }
}

pub struct Command {
    command: Option<Commands>,
    raw_command: String,
    // for debug, since we can't just add debug prints
    debug_status: String,
}

impl Command {
    pub fn new(line: String) -> Self {
        let mut args: Vec<String> = line
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let command = match args
        .pop()
        .expect("Program Error if nothing there")
        .strip_prefix("/")
        .unwrap()
        .to_lowercase()
        .as_str() {
            s => match Commands::from_str(s) {
                Ok(c) => Some(c),
                _ => None,
            }
        };
        Self {
            command,
            raw_command: line,
            debug_status: "".to_string(),
        }
    }

    pub fn run(&self, screen: &mut Screen) {
        // Matching on the Commands enum
        match &self.command {
            Some(c) => screen.set_status(&format!("{}", &c.name().to_uppercase())),
            _ => {
                if self.debug_status.len() > 0 {
                    screen.set_status(&self.debug_status);
                } else {
                    screen.set_status(&format!("Unmatched command {}", &self.raw_command));
                }
            }
        }
    }

    pub fn suggest(&self) -> bool {
        false
    }
}
