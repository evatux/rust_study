// Game drawer

extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use utypes::Pos;
use utypes::Board;
use game::Snake;
use game::Game;

type Screen = termion::raw::RawTerminal<std::io::Stdout>;

const SYMBOL_EMPRY: &str = " ";
const SYMBOL_BORDER: &str = "█";
const SYMBOL_SNAKE_BODY: &str = "o";
const SYMBOL_SNAKE_HEAD: &str = "@";
const SYMBOL_FOOD: &str = "¤";


pub struct GameDrawer {
    screen: Screen,
    base_offset: Pos,   // game canvas offset
    board_offset: Pos,  // game board offset ( > base)
}

impl Pos {
    fn into_cursor_pos(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x + 1, self.y + 1)
    }
}

impl GameDrawer {
    pub fn get_max_board_size() -> Board {
        let terminal_sizes = termion::terminal_size().unwrap();
        let reserve = Board{x: 10, y: 10};
        Board{x: terminal_sizes.0, y: terminal_sizes.1} - reserve
    }

    pub fn new(game: &Game) -> GameDrawer {
        let max_board_size = Self::get_max_board_size();

        assert!(game.board.x <= max_board_size.x);
        assert!(game.board.y <= max_board_size.y);

        GameDrawer {
            screen: stdout().into_raw_mode().unwrap(),
            base_offset: Pos{x: 1, y: 1},
            board_offset: Pos{x: 3, y: 3},
        }
    }

    pub fn init(&mut self, game: &Game) {
        write!(self.screen, "{}", termion::clear::All);
        self.draw_border(game);
        self.draw_snake(game);
    }

    pub fn fini(&mut self, game: &Game) {
        write!(self.screen, "{}\n\r{}",
               (self.board_offset + game.board).into_cursor_pos(),
               termion::style::Reset);
    }

    /* private methods */

    fn print_at_pos(&mut self, pos: Pos, s: &str) {
        write!(self.screen, "{}{}", pos.into_cursor_pos(), s)
            .unwrap();
    }

    fn draw_border(&mut self, game: &Game) {
        let bs = SYMBOL_BORDER;

        let border_base = self.board_offset - Pos{x: 1, y: 1};
        let border = game.board + Board{x: 2, y: 2};

        self.print_at_pos(border_base, "");
        for _ in 0 .. border.x {
            write!(self.screen, "{}", &bs).unwrap();
        }

        for y in 1 .. border.y - 1 {
            self.print_at_pos(border_base + Pos{x: 0, y}, &bs);
            self.print_at_pos(border_base + Pos{x: border.x - 1, y}, &bs);
        }

        self.print_at_pos(border_base + Pos{x: 0, y: border.y - 1}, "");
        for _ in 0 .. border.x {
            write!(self.screen, "{}", &bs).unwrap();
        }
    }

    fn draw_snake(&mut self, game: &Game) {
        let board_offset = self.board_offset;

        for p in (&game.snake).into_iter().take(1) {
            self.print_at_pos(board_offset + p, SYMBOL_SNAKE_HEAD);
        }
        for p in (&game.snake).into_iter().skip(1) {
            self.print_at_pos(board_offset + p, SYMBOL_SNAKE_BODY);
        }
    }
}