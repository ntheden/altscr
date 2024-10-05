use crate::screen::Screen;
use std::str::FromStr;
use strum_macros::EnumString;
use strum::{VariantNames, IntoStaticStr};
use strprox::Autocompleter;

#[derive(Debug, EnumString, strum_macros::VariantNames, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
enum Command {
    Clear,
    #[strum(serialize = "exit")]
    Exit(NoArg),
    #[strum(serialize = "help", serialize = "h", serialize = "?")]
    Help,
    Menu,
    #[strum(serialize = "quit", serialize = "q")]
    Quit,
}

// Testing out stuff
#[derive(Debug, Default)]
struct NoArg {
}

pub struct Commands {
    command: Option<Command>,
    raw_line: String,
    args: Vec<String>,
    raw_command: String, // Without the "/" prefix
    // for debug, since we can't just add debug prints
    debug_status: String,
}

impl Commands {
    pub fn new(line: String) -> Self {
        let mut args: Vec<String> = line
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let raw_command = args
            .pop()
            .expect("Program Error if nothing there")
            .strip_prefix("/")
            .unwrap()
            .to_lowercase()
            .to_string();
        let command = if raw_command.len() > 0 {
            match Command::from_str(raw_command.as_str()) {
                Ok(c) => Some(c),
                _ => None,
            }
        } else {
            None
        };
        Self {
            command, // Commands holds one command.
            raw_line: line,
            raw_command,
            args,
            debug_status: format!("{:?}", Command::VARIANTS),
        }
    }

    pub fn run(&mut self, screen: &mut Screen) {
        // Matching on the Command enum
        match &self.command {
            Some(c) => screen.set_status(&format!("{}", &self.raw_command.to_uppercase())),
            _ => {
                if self.debug_status.len() > 0 {
                    screen.set_status(&self.debug_status);
                    self.debug_status.clear();
                } else {
                    screen.set_status(&format!("Unmatched command {}", &self.raw_command));
                }
            }
        }
    }

    pub fn suggest(&self, screen: &mut Screen) {
        match &self.command {
            Some(c) => {
                let name: &'static str = c.into();
                screen.set_status(&format!("{}", name));
            }
            _ => {
                if self.raw_command.len() > 0 {
                    let autocompleter = Autocompleter::new(&Command::VARIANTS);
                    let result = autocompleter.autocomplete(&self.raw_command, 1);
                    let result_strings: Vec::<&str> = result
                        .iter()
                        .map(|measured_prefix| measured_prefix.string.as_str())
                        .collect();
                    screen.set_status(&format!("/{}", result_strings[0]));
                } else {
                    screen.set_status("/");
                }
            }
        }
    }
}
