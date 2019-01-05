// Game drawer

extern crate termion;

use std::io::{Write, stdout};

use termion::raw::IntoRawMode;

use utypes::Pos;
use utypes::Board;
use game::Game;
use game::GameUpdate;

const SYMBOL_EMPTY: &str = " ";
const SYMBOL_BORDER: &str = "█";
const SYMBOL_BORDER_PERIODIC: &str = "▒";
const SYMBOL_SNAKE_BODY: &str = "o";
const SYMBOL_SNAKE_HEAD: &str = "@";
const SYMBOL_FOOD: &str = "¤";

type Screen = termion::raw::RawTerminal<std::io::Stdout>;

pub struct GameDrawer {
    screen: Screen,
    base_offset: Pos,   // game canvas offset
    board_offset: Pos,  // game board offset ( > base)
}

impl Pos {
    fn into_cursor_pos(&self) -> termion::cursor::Goto {
        termion::cursor::Goto((self.x + 1) as u16, (self.y + 1) as u16)
    }
}

impl GameDrawer {
    pub fn get_max_board_size() -> Board {
        let terminal_sizes = termion::terminal_size().unwrap();
        let reserve = Board{x: 10, y: 10};
        Board{x: terminal_sizes.0 as i16, y: terminal_sizes.1 as i16} - reserve
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
        write!(self.screen, "{}{}",
               termion::clear::All,
               termion::cursor::Hide);
        self.draw_border(game);
        self.draw_snake(game);
        self.draw_food(game);
        self.flush();
    }

    pub fn fini(&mut self, game: &Game) {
        write!(self.screen, "{}\n\r{}{}\nGame over!\n\r\n",
               (self.board_offset + game.board).into_cursor_pos(),
               termion::style::Reset, termion::cursor::Show);
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    pub fn update_scene(&mut self, game: &Game, update: &GameUpdate) {
        if let Some(pos) = update.head_prev_pos {
            self.board_print_at_pos(pos, SYMBOL_SNAKE_BODY);
        }

        if let Some(pos) = update.tail_prev_pos {
            self.board_print_at_pos(pos, SYMBOL_EMPTY);
        }

        if update.food_renew {
            self.board_print_at_pos(game.food.pos, SYMBOL_FOOD);
        }

        self.board_print_at_pos(game.snake.head(), SYMBOL_SNAKE_HEAD);
        self.flush();
    }

    /* private methods */

    fn board_print_at_pos(&mut self, pos: Pos, s: &str) {
        let pos = self.board_offset + pos;
        self.print_at_pos(pos, s);
    }

    fn print_at_pos(&mut self, pos: Pos, s: &str) {
        write!(self.screen, "{}{}", pos.into_cursor_pos(), s)
            .unwrap();
    }

    fn draw_border(&mut self, game: &Game) {
        let bs = if game.periodic_world {
            SYMBOL_BORDER_PERIODIC
        } else {
            SYMBOL_BORDER
        };

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

    fn draw_food(&mut self, game: &Game) {
        let board_offset = self.board_offset;
        self.print_at_pos(board_offset + game.food.pos, SYMBOL_FOOD);
    }
}
