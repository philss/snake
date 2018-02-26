#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Right,
    Left
}

#[derive(Clone, Copy, Debug)]
pub struct BodySection {
    pos_x: i64,
    pos_y: i64,
    direction: Direction,
}

#[derive(Clone, Debug)]
pub struct Snake {
    head: BodySection,
    tail: Vec<BodySection>,
}

impl Snake {
    pub fn new(x: i64, y: i64) -> Snake {
        let tail: Vec<BodySection> = vec![];
        let head: BodySection = BodySection::new(x, y, Direction::Up);
        Snake { head: head, tail: tail }
    }

    pub fn size(&self) -> usize {
        self.tail.len()
    }

    pub fn head_position(&self) -> (i64, i64) {
        self.head.position()
    }

    pub fn walk(self, direction: Direction) -> Self {
        let pos_x = self.head.pos_x;
        let pos_y = self.head.pos_y;
        let head = match direction {
            Direction::Left => BodySection::new(pos_x - 1, pos_y, direction),
            Direction::Right => BodySection::new(pos_x + 1, pos_y, direction),
            Direction::Up => BodySection::new(pos_x, pos_y + 1, direction),
            Direction::Down => BodySection::new(pos_x, pos_y + -1, direction),
        };
        let mut tail = Vec::with_capacity(self.size() + 1);
        let mut last_body_section = self.head;

        for body_section in self.tail {
            tail.push(
                BodySection::new(
                    last_body_section.pos_x,
                    last_body_section.pos_y,
                    last_body_section.direction
                    )
                );

            last_body_section = body_section;
        }

        Snake { head: head, tail: tail }
    }
}

impl BodySection {
    pub fn new(x: i64, y: i64, dir: Direction) -> BodySection {
        BodySection { pos_x: x, pos_y: y, direction: dir }
    }

    pub fn position(self) -> (i64, i64) {
        (self.pos_x, self.pos_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_properties() {
        let snake = Snake::new(0, 0);

        assert_eq!(snake.size(), 0);
        assert_eq!(snake.head_position(), (0, 0));

        let tail_2: Vec<BodySection> = vec![BodySection::new(1, 2, Direction::Down)];

        let head_2 = BodySection::new(0, 0, Direction::Up);
        let snake_2 = Snake { head: head_2, tail: tail_2 };

        assert_eq!(snake_2.size(), 1);
    }

    #[test]
    fn it_moves_snake_to_left() {
        let snake = Snake::new(10, 20);
        let snake_1 = snake.walk(Direction::Left);

        assert_eq!(snake_1.head_position(), (9, 20));

        let tail_2: Vec<BodySection> = vec![
            BodySection::new(20, 19, Direction::Up),
            BodySection::new(20, 18, Direction::Up)
            ];
        let snake_2 = Snake { head: BodySection::new(20, 20, Direction::Left), tail: tail_2 };
        let snake_2 = snake_2.walk(Direction::Left);

        assert_eq!(snake_2.head_position(), (19, 20));

        let first = snake_2.tail[0];
        let second = snake_2.tail[1];

        assert_eq!(first.position(), (20, 20));
        assert_eq!(second.position(), (20, 19));
    }

    #[test]
    fn it_moves_snake_to_right() {
        let snake = Snake::new(10, 20);
        let snake_1 = snake.walk(Direction::Right);

        assert_eq!(snake_1.head_position(), (11, 20));

        let tail_2: Vec<BodySection> = vec![
            BodySection::new(20, 19, Direction::Up),
            BodySection::new(20, 18, Direction::Up)
            ];
        let snake_2 = Snake { head: BodySection::new(20, 20, Direction::Right), tail: tail_2 };
        let snake_2 = snake_2.walk(Direction::Right);

        assert_eq!(snake_2.head_position(), (21, 20));

        let first = snake_2.tail[0];
        let second = snake_2.tail[1];

        assert_eq!(first.position(), (20, 20));
        assert_eq!(second.position(), (20, 19));
    }

    #[test]
    fn it_moves_to_up_and_down() {
        let snake = Snake::new(10, 20);
        let snake = snake.walk(Direction::Up);

        assert_eq!(snake.head_position(), (10, 21));

        let snake_2 = Snake::new(10, 20);
        let snake_2 = snake_2.walk(Direction::Down);

        assert_eq!(snake_2.head_position(), (10, 19));
    }
}
