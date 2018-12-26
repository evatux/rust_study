extern crate termion;

mod utypes;
mod game;
mod draw;

use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use utypes::Pos;
use utypes::Board;
use game::Game;
use draw::GameDrawer;

type Screen = termion::raw::RawTerminal<std::io::Stdout>;

fn play() {
    let mut game = Game::new(Pos{x: 16, y: 16}, 4, false);
    let mut drawer = GameDrawer::new(&game);

    drawer.init(&game);

    drawer.fini(&game);
    // stdout.flush().unwrap();
}

fn main() {
    play();
}

#[allow(dead_code)]
fn xmain() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
            .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
