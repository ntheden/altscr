use std::io::{Write, stdin, stdout, Stdout};
use termion::screen::{IntoAlternateScreen, AlternateScreen};
use termion::terminal_size;
use termion::raw::{RawTerminal, IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;
use termion::{cursor, style, color};

use crate::command::Command;
use crate::record::{Record, Records};


pub struct Screen {
    pub stdout: AlternateScreen<RawTerminal<Stdout>>,
    x_offset: usize,
    prompt: String,
    records: Records,
    current_input: Vec<char>,
    command_mode: bool,
    status_bar_text: String,
    title_bar_text: String,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap(),
            x_offset: 0,
            prompt: "<user>".to_string(),
            records: Records::new(),
            current_input: Vec::new(),
            command_mode: false,
            status_bar_text: "Status Bar".to_string(),
            title_bar_text: "Title Bar".to_string(),
        }
    }

    pub fn set_status(&mut self, text: &str) {
        self.status_bar_text = text.to_string();
    }

    fn term_size() -> (usize, usize) {
        let (width, height) = terminal_size().unwrap();
        (width as usize, height as usize)
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap()
    }

    fn write(&mut self, data: &str) {
        write!(self.stdout, "{}", data).unwrap()
    }

    pub fn write_flush(&mut self, data: &str) {
        self.write(data);
        self.flush()
    }

    fn current_line(&self) -> String {
        self.current_input.iter().collect::<String>()
    }

    pub fn main_loop(&mut self) {
        self.home();
        let stdin = stdin();
        for k in stdin.keys() {
            match k.as_ref().unwrap() {
                Key::Char('/') => {
                    print!("/");
                    self.current_input.push('/');
                    self.offset(1);
                    if self.current_input.len() == 1 {
                        self.command_mode = true;
                    }
                }
                Key::Char('\n') => {
                    // ENTER
                    if self.command_mode {
                        let mut command = Command::new(self.current_line());
                        command.run(self); // remove mut: command is mut for debug_status
                        self.command_mode = false;
                    } else {
                        let record = Record::from_str(self.current_line());
                        self.records.push(record);
                    }
                    self.current_input.clear();
                    self.home();
                }
                Key::Ctrl('d') => break,
                Key::Backspace => self.offset(-1),
                Key::Char(c) => {
                    print!("{}", c);
                    self.current_input.push(*c);
                    self.offset(1);
                    if self.command_mode {
                        let command = Command::new(self.current_line());
                        command.suggest();
                    }
                }
                _ => print!("{:?}", k)
            }
        }
        self.flush();
    }

    fn offset(&mut self, x: i16) {
        if x < 0 {
            self.current_input.pop();
            self.x_offset -= x.abs() as usize;
        } else {
            self.x_offset += x as usize;
        }
        self.home();
    }

    /// The records that are in view
    fn records_view(&mut self) -> String {
        let (_xsize, ysize) = Self::term_size();
        let mut buffer = String::new();
        let start_index: usize =
            Record::count_newlines(&self.current_line())
            + 1; // status bar
        let view_ysize: usize =
            ysize
            - start_index;
        for (index, record) in &mut self.records
            .iter()
            .rev() // newest at the bottom!
            .enumerate() {
            let record_index =
                index
                + start_index;
            if  record_index > view_ysize {
                // todo: better ux if a partial record is shown
                break;
            }
            buffer.push_str(
                &format!(
                    "{}{}{}",
                    Self::goto(0, ysize - record_index),
                    &format!("<{}>", record.username),
                    record.data.clone() + "\n",
                )
            );
        }
        buffer
    }

    fn goto(x: usize, y: usize) -> cursor::Goto {
        cursor::Goto(x.try_into().unwrap(), y.try_into().unwrap())
    }

    fn bar(&self, text: &str, max_width: usize) -> String {
        format!("{: <1$}", text, max_width)
    }

    fn home(&mut self) {
        self.write(&format!("{}", termion::clear::All,));
        let (xsize, y_offset) = Self::term_size();

        let records = self.records_view();
        self.write_flush(
            &format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                // title bar
                Self::goto(0, 1),
                color::Bg(color::Blue),
                &self.bar(&self.title_bar_text, xsize),
                style::Reset,
                records,
                // status bar
                Self::goto(0, y_offset - 1),
                color::Bg(color::Blue),
                &self.bar(&self.status_bar_text, xsize),
                Self::goto(0, y_offset),
                // prompt section
                style::Reset,
                &self.prompt,
                &self.current_line(),
            ) 
        );
    }
}
