mod command;
mod record;
mod screen;
use screen::Screen;

fn main() {
    let mut screen = Screen::new();
    screen.main_loop();
    println!("Finished.");
}
