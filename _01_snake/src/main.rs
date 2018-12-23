extern crate termion;

mod pos;
mod snake;

use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use pos::Pos;
use snake::Snake;

type Screen = termion::raw::RawTerminal<std::io::Stdout>;

impl Pos {
    fn into_cursor_pos(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.1 + 1, self.0 + 1)
    }
}

fn print_at_pos(stdout: &mut Screen, offset: Pos, s: &str) {
    write!(stdout, "{}{}", offset.into_cursor_pos(), s).unwrap();
}

fn draw_border(stdout: &mut Screen, size: Pos, offset: Pos, c: char) {
    let bs = c.to_string();

    write!(stdout, "{}", offset.into_cursor_pos()).unwrap();
    for _ in 0 .. size.1 {
        write!(stdout, "{}", &bs).unwrap();
    }

    for i in 1 .. size.0 - 1 {
        print_at_pos(stdout, &offset + &Pos(i, 0), &bs);
        print_at_pos(stdout, &offset + &Pos(i, size.1 - 1), &bs);
    }

    write!(stdout, "{}", (&offset + &Pos(size.0 - 1, 0)).into_cursor_pos())
           .unwrap();
    for _ in 0 .. size.1 {
        write!(stdout, "{}", &bs).unwrap();
    }
}

fn foo() {
    let _stdin = stdin();
    let mut stdout: Screen = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All);

    draw_border(&mut stdout, Pos(10, 10), Pos(16, 3), '█');

    write!(stdout, "{}\n\r", termion::style::Reset);
    stdout.flush().unwrap();
}

fn main() {
    foo();

    let mut snake = Snake::with_capacity(40, Pos(1, 1));
    snake.step(Pos(1, 2));
    snake.grow(Pos(2, 2));
    println!("Snake tail is at {:?}", snake.tail());
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
