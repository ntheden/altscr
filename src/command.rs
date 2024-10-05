use strprox::Autocompleter;
use crate::screen::Screen;
use casey::lower;

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

pub struct Command {
    command: Option<Commands>,
    raw_command: String,
}

impl Command {
    pub fn new(line: String) -> Self {
        let command = match line
            .trim()
            .strip_prefix("/")
            .unwrap()
            .to_lowercase()
            .as_str() {
            _ => None,
        };
        Self {
            command: command,
            raw_command: line,
        }
    }

    pub fn run(&self, screen: &mut Screen) {
        match self.command {
            _ => screen.set_status("hi")
        }
    }

    pub fn suggest(&self) -> bool {
        false
    }
}
