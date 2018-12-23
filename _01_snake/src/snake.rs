// Snake data type

use pos::Pos;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Pos>,
    size: usize,
    head_idx: usize,
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

    pub fn head(&self) -> &Pos {
        &self.body[self.head_idx]
    }

    pub fn tail(&self) -> &Pos {
        &self.body[self.tail_idx()]
    }

    pub fn step(&mut self, pos: Pos) {
        let tidx = self.tail_idx();
        self.body[tidx] = pos;
        self.head_idx = tidx;
    }

    pub fn grow(&mut self, new_head: Pos) {
        self.body.insert(self.head_idx, new_head);
        self.size += 1;
    }

    fn tail_idx(&self) -> usize {
        (self.size + self.head_idx - 1) % self.size
    }
}

#[test]
fn simple() {
    let mut snake = Snake::with_capacity(40, Pos(1, 1));
    assert_eq!(snake.head(), &Pos(1, 1));
    snake.step(Pos(1, 2));
    assert_eq!(snake.head(), &Pos(1, 2));
    snake.grow(Pos(2, 2));
    assert_eq!(snake.head(), &Pos(2, 2));
    assert_eq!(snake.tail(), &Pos(1, 2));
    assert_eq!(snake.size, 2);
}
