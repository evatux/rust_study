// Game state and others

extern crate rand;

use std::cmp;

use self::rand::Rng;

use utypes::Pos;
use utypes::Board;

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Move(Dir),
    Nop,
    Exit,
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Blue,
    Green,
    Red,
}

pub struct Food {
    pub pos: Pos,
}

#[derive(Debug)]
pub struct Snake {
    body: Vec<Pos>,
    head_idx: usize,
    pub dir: Dir,
    pub color: Color,
}

pub struct Game {
    pub board: Board,
    pub snake: Snake,
    pub food: Food,
    pub periodic_world: bool,
}

pub struct GameUpdate {
    pub head_prev_pos: Option<Pos>,
    pub tail_prev_pos: Option<Pos>,
    pub food_renew: bool,
}

impl Dir {
    fn into_pos(&self) -> Pos {
        match self {
            Dir::Down => Pos{x: 0, y: 1},
            Dir::Left => Pos{x: -1, y: 0},
            Dir::Right => Pos{x: 1, y: 0},
            Dir::Up => Pos{x: 0, y: -1},
        }
    }
}

impl Snake {
    pub fn with_capacity(size: usize, head: Pos) -> Snake {
        let mut s = Snake {
            body: Vec::with_capacity(size),
            head_idx: 0,
            dir: Dir::Right,
            color: Color::Green,
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

    pub fn grow(&mut self, new_head: Pos) {
        self.body.insert(self.head_idx, new_head);
    }

    pub fn contains(&self, pos: Pos) -> bool {
        self.body.contains(&pos)
    }

    fn tail_idx(&self) -> usize {
        (self.body.len() + self.head_idx - 1) % self.body.len()
    }
}

impl Game {
    pub fn new(board: Board, snake_len: u16, periodic_world: bool) -> Game {
        assert!(board.x > 4 && board.y > 4);
        assert!(snake_len < i16::max_value() as u16);

        let mut rng = rand::thread_rng();

        let snake_len = snake_len as i16;
        let snake_len = cmp::min(snake_len, board.x - 2);
        let snake_pos = Pos {
            x: rng.gen_range(0, (board.x - snake_len) / 2),
            y: rng.gen_range(0, board.y),
        };

        let capacity = 2usize * board.x as usize * board.y as usize;
        let mut snake = Snake::with_capacity(capacity, snake_pos);

        let grow_dir_vec = snake.dir.into_pos();
        for l in 1 .. snake_len {
            snake.grow(snake_pos + l * grow_dir_vec)
        }

        let mut game = Game {
            board,
            snake,
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

    pub fn exec(&mut self, cmd: Command) -> Option<GameUpdate> {
        match cmd {
            Command::Move(dir) => self.step(dir),
            Command::Nop => {
                let dir = self.snake.dir;
                self.step(dir)
            }
            Command::Exit => None,
        }
    }

    fn normalize_dir(&self, dir: Dir) -> Dir {
        let snake_dir_vec = self.snake.dir.into_pos();
        let vec = dir.into_pos();

        match vec + snake_dir_vec {
            Pos{x: 0, y: 0} => self.snake.dir,
            _ => dir,
        }
    }

    fn step(&mut self, dir: Dir) -> Option<GameUpdate> {
        let dir = self.normalize_dir(dir);
        self.snake.dir = dir;

        let head_cur_pos = self.snake.head();
        let mut head_new_pos = head_cur_pos + dir.into_pos();

        let periodic = self.periodic_world;

        // check board bounds
        if head_new_pos.x < 0 {
            if periodic { head_new_pos.x += self.board.x } else { return None }
        }
        if head_new_pos.y < 0 {
            if periodic { head_new_pos.y += self.board.y } else { return None }
        }
        if head_new_pos.x >= self.board.x {
            if periodic { head_new_pos.x -= self.board.x } else { return None }
        }
        if head_new_pos.y >= self.board.y {
            if periodic { head_new_pos.y -= self.board.y } else { return None }
        }

        if head_new_pos == self.food.pos {
            self.snake.grow(head_new_pos);
            self.generate_food();

            return Some(GameUpdate{
                head_prev_pos: Some(head_cur_pos),
                tail_prev_pos: None,
                food_renew: true,
            });
        }

        if !self.snake.can_step(head_new_pos) {
            return None
        }

        let tail_cur_pos = self.snake.tail();
        self.snake.step(head_new_pos);

        Some(GameUpdate{
            head_prev_pos: Some(head_cur_pos),
            tail_prev_pos: Some(tail_cur_pos),
            food_renew: false,
        })
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
        if self.index == self.snake.body.len() {
            return None;
        }

        let idx = (self.snake.head_idx + self.index) % self.snake.body.len();
        self.index += 1;

        Some(self.snake.body[idx])
    }
}

#[test]
fn snake_simple_test() {
    let mut snake = Snake::with_capacity(40, Pos{x: 1, y: 1});
    assert_eq!(snake.head(), Pos{x: 1, y: 1});
    snake.step(Pos{x: 1, y: 2});
    assert_eq!(snake.head(), Pos{x: 1, y: 2});
    snake.grow(Pos{x: 2, y: 2});
    assert_eq!(snake.head(), Pos{x: 2, y: 2});
    assert_eq!(snake.tail(), Pos{x: 1, y: 2});
    assert_eq!(snake.body.len(), 2);
    snake.grow(Pos{x: 2, y: 3});
    snake.grow(Pos{x: 1, y: 3});
    assert!(snake.can_step(Pos{x: 1, y: 2}));
    assert!(!snake.can_step(Pos{x: 2, y: 3}));
    assert!(snake.contains(Pos{x: 2, y: 3}));
    assert!(!snake.contains(Pos{x: 1, y: 1}));

    let mut iter = IntoIterator::into_iter(&snake);
    assert_eq!(Pos{x: 1, y: 3}, iter.next().unwrap());
    assert_eq!(Pos{x: 2, y: 3}, iter.next().unwrap());
    assert_eq!(Pos{x: 2, y: 2}, iter.next().unwrap());
    assert_eq!(Pos{x: 1, y: 2}, iter.next().unwrap());
    assert_eq!(None, iter.next());
}
