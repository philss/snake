#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Direction {
    Down,
    Up,
    Right,
    Left
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct BodySection {
    pos_x: i64,
    pos_y: i64,
    direction: Direction,
}

#[allow(dead_code)]
struct Snake {
    head: BodySection,
    tail: Vec<BodySection>,
}

impl Snake {
    #[allow(dead_code)]
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

    pub fn walk(&self, direction: Direction) -> Snake {
        let head = match direction {
            Direction::Left => BodySection::new(self.head.pos_x - 1, self.head.pos_y, direction),
            _ => self.head.clone(),
        };
        let mut tail = Vec::with_capacity(self.size() + 1);
        let mut last_body_section = self.head.clone();

        for body_section in &self.tail {
            tail.push(
                BodySection::new(
                    last_body_section.pos_x,
                    last_body_section.pos_y,
                    last_body_section.direction
                    )
                );

            last_body_section = body_section.clone();
        }

        Snake { head: head, tail: tail }
    }
}

impl BodySection {
    pub fn new(x: i64, y: i64, dir: Direction) -> BodySection {
        BodySection { pos_x: x, pos_y: y, direction: dir }
    }

    pub fn position(&self) -> (i64, i64) {
        (self.pos_x, self.pos_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_properties() {
        let snake: Snake = Snake::new(0, 0);

        assert_eq!(snake.size(), 0);
        assert_eq!(snake.head_position(), (0, 0));

        let body = BodySection::new(1, 2, Direction::Down);
        let tail_2: Vec<BodySection> = vec![body];
        let head_2: BodySection = BodySection { pos_x: 0, pos_y: 0, direction: Direction::Up };
        let snake_2: Snake = Snake { head: head_2, tail: tail_2 };
        assert_eq!(snake_2.size(), 1);
    }

    #[test]
    fn it_moves_snake_to_left() {
        let snake = Snake::new(10, 20);

        let snake_1 = snake.walk(Direction::Left);
        assert_eq!(snake_1.head_position(), (9, 20));

        let body_1 = BodySection::new(20, 19, Direction::Up);
        let body_2 = BodySection::new(20, 18, Direction::Up);
        let tail_2: Vec<BodySection> = vec![body_1, body_2];
        let head_2: BodySection = BodySection { pos_x: 20, pos_y: 20, direction: Direction::Left };

        let snake_2: Snake = Snake { head: head_2, tail: tail_2 };
        let snake_3 = snake_2.walk(Direction::Left);
        assert_eq!(snake_3.head_position(), (19, 20));

        let first = &snake_3.tail[0];
        let second = &snake_3.tail[1];

        assert_eq!(first.position(), (20, 20));
        assert_eq!(second.position(), (20, 19));
    }
}
