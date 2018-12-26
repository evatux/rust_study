// Game state and others

extern crate rand;

use std::cmp;

use self::rand::Rng;

use utypes::Pos;
use utypes::Board;

pub enum Dir {
    Down,
    Left,
    Right,
    Up,
}

pub enum Command {
    Move(Dir),
    Exit,
}

pub struct Food {
    pub pos: Pos,
}

#[derive(Debug)]
pub struct Snake {
    body: Vec<Pos>,
    size: usize,
    head_idx: usize,
}

pub struct Game {
    pub board: Board,
    pub snake: Snake,
    pub current_snake_dir: Dir,
    pub food: Food,
    pub periodic_world: bool,
}

pub struct GameSceneUpdate {
    pub new_head_pos: Pos,
    pub prev_head_pos: Pos,
    pub prev_head_now_body: bool,
    pub prev_tail_pos: Pos,
    pub prev_tail_remains: bool,
    pub food_regenerated: bool,
}

impl Snake {
    pub fn with_capacity(size: usize, head: Pos) -> Snake {
        let mut s = Snake {
            body: Vec::with_capacity(size),
            size: 1,
            head_idx: 0,
        };
        s.body.push(head);
        s
    }

    pub fn head(&self) -> Pos {
        self.body[self.head_idx]
    }

    pub fn tail(&self) -> Pos {
        self.body[self.tail_idx()]
    }

    pub fn can_step(&self, pos: Pos) -> bool {
        match self.body.iter().position(|ref x| x == &&pos) {
            None => true,
            Some(v) => v == self.tail_idx(),
        }
    }

    pub fn step(&mut self, pos: Pos) {
        let tidx = self.tail_idx();
        self.body[tidx] = pos;
        self.head_idx = tidx;
    }

    pub fn can_grow(&self, pos: Pos) -> bool {
        !self.body.contains(&pos)
    }

    pub fn grow(&mut self, new_head: Pos) {
        self.body.insert(self.head_idx, new_head);
        self.size += 1;
    }

    pub fn contains(&self, pos: Pos) -> bool {
        self.body.contains(&pos)
    }

    fn tail_idx(&self) -> usize {
        (self.size + self.head_idx - 1) % self.size
    }
}

impl Game {
    pub fn new(board: Board, snake_len: u16, periodic_world: bool) -> Game {
        assert!(board.x > 4 && board.y > 4);

        let mut rng = rand::thread_rng();

        let snake_len = cmp::min(snake_len, board.x - 2);
        let snake_pos = Pos {
            x: rng.gen_range(0, (board.x - snake_len) / 2),
            y: rng.gen_range(0, board.y),
        };

        let mut snake = Snake::with_capacity(
            2usize * board.x as usize * board.y as usize, snake_pos);

        for x in 1 .. snake_len {
            snake.grow(snake_pos + Pos{x, y: 0})
        }

        let mut game = Game {
            board,
            snake,
            current_snake_dir: Dir::Left,
            food: Food { pos: Pos{x: 0, y: 0} }, // tentative
            periodic_world
        };
        game.generate_food();

        game
    }

    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let pos = Pos {
                x: rng.gen_range(0, self.board.x),
                y: rng.gen_range(0, self.board.y),
            };

            if !self.snake.contains(pos) {
                self.food = Food{ pos };
                break;
            }
        }
    }

    pub fn exec(&mut self, cmd: Command) -> Option<GameSceneUpdate> {
        match cmd {
            Command::Move(dir) => self.step(dir),
            Command::Exit => None,
        }
    }

    fn step(&mut self, dir: Dir) -> Option<GameSceneUpdate> {
        None
    }
}

impl<'a> IntoIterator for &'a Snake {
    type Item = Pos;
    type IntoIter = SnakeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SnakeIterator { snake: self, index: 0 }
    }
}

pub struct SnakeIterator<'a> {
    snake: &'a Snake,
    index: usize,
}

impl<'a> Iterator for SnakeIterator<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        if self.index == self.snake.size {
            return None;
        }

        let idx = (self.snake.head_idx + self.index) % self.snake.size;
        self.index += 1;

        Some(self.snake.body[idx])
    }
}

#[test]
fn simple() {
    let mut snake = Snake::with_capacity(40, Pos{x: 1, y: 1});
    assert_eq!(snake.head(), Pos{x: 1, y: 1});
    snake.step(Pos{x: 1, y: 2});
    assert_eq!(snake.head(), Pos{x: 1, y: 2});
    snake.grow(Pos{x: 2, y: 2});
    assert_eq!(snake.head(), Pos{x: 2, y: 2});
    assert_eq!(snake.tail(), Pos{x: 1, y: 2});
    assert_eq!(snake.size, 2);
    snake.grow(Pos{x: 2, y: 3});
    snake.grow(Pos{x: 1, y: 3});
    assert!(snake.can_step(Pos{x: 1, y: 2}));
    assert!(snake.can_grow(Pos{x: 1, y: 4}));
    assert!(!snake.can_step(Pos{x: 2, y: 3}));
    assert!(!snake.can_grow(Pos{x: 1, y: 2}));
    assert!(snake.contains(Pos{x: 2, y: 3}));
    assert!(!snake.contains(Pos{x: 1, y: 1}));

    let mut iter = IntoIterator::into_iter(&snake);
    assert_eq!(Pos{x: 1, y: 3}, iter.next().unwrap());
    assert_eq!(Pos{x: 2, y: 3}, iter.next().unwrap());
    assert_eq!(Pos{x: 2, y: 2}, iter.next().unwrap());
    assert_eq!(Pos{x: 1, y: 2}, iter.next().unwrap());
    assert_eq!(None, iter.next());
}
