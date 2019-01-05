extern crate termion;

mod utypes;
mod game;
mod draw;

use std::{thread, time};

use termion::event::Key;
use termion::input::TermRead;

use utypes::Board;
use game::Game;
use draw::GameDrawer;

fn play() {
    let mut stdin = termion::async_stdin().keys();

    let mut game = Game::new(Board{x: 16, y: 16}, 4, true);
    let mut drawer = GameDrawer::new(&game);

    drawer.init(&game);

    loop {
        use game::Dir;
        use game::Command;

        let key = stdin.by_ref().last();

        let cmd = match key {
            Some(Ok(Key::Char('q'))) => Command::Exit,
            Some(Ok(Key::Left)) => Command::Move(Dir::Left),
            Some(Ok(Key::Right)) => Command::Move(Dir::Right),
            Some(Ok(Key::Up)) => Command::Move(Dir::Up),
            Some(Ok(Key::Down)) => Command::Move(Dir::Down),
            _ => Command::Nop,
        };

        let update = game.exec(cmd);
        if let Some(update) = update {
            drawer.update_scene(&game, &update);
        } else {
            drawer.fini(&game);
            break;
        }

        thread::sleep(time::Duration::from_millis(500));
    }

    drawer.fini(&game);
}

fn main() {
    play();
}
