use crate::direction::Direction;

pub struct Snake {
    pub body: Vec<(u8, u8)>,
    pub direction: Direction,
    next_direction: Option<Direction>,
}

impl Snake {
    pub fn new(body: Vec<(u8, u8)>, direction: Direction) -> Snake {
        Snake {
            body,
            direction,
            next_direction: None,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up | Direction::Down => match self.direction {
                Direction::Left | Direction::Right => self.next_direction = Some(direction),
                _ => {}
            },
            Direction::Left | Direction::Right => match self.direction {
                Direction::Up | Direction::Down => self.next_direction = Some(direction),
                _ => {}
            },
        }
    }

    pub fn next_direction(&mut self) -> Direction {
        match self.next_direction {
            Some(direction) => {
                self.direction = direction;
                self.next_direction = None;
            }
            None => {}
        }

        self.direction
    }
}
