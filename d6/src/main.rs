use crate::Directions::*;

fn main() {
    let file = std::fs::read_to_string("input2").unwrap();
    print!("{}", file);
    let lines: Vec<&str> = file.split('\n').collect();
    let mut chars: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    let starting_position_y = chars.iter().position(|x| x.contains(&'^')).unwrap();
    let starting_position_x = chars[starting_position_y]
        .iter()
        .position(|x| x == &'^')
        .unwrap();
    let mut direction = Direction::create(Up);
    let mut position = Position::create(starting_position_x, starting_position_y);

    let mut total = 0;
    let mut total2 = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] != '#' && chars[i][j] != '^' {
                let mut chars2 = chars.clone();
                chars2[i][j] = '#';
                if check(&mut position, &mut direction, &mut chars2) {
                    total += 1;
                }
            }
            total2 += 1;
            println!("{total2}");
        }
    }
    println!("Total: {total}");
}

fn check(position: &mut Position, direction: &mut Direction, chars: &mut Vec<Vec<char>>) -> bool {
    let mut counter = 0;
    loop {
        counter += 1;
        if get_moved_pos(&position, &direction).get_char(&chars) == '#' {
            (direction).turn();
        }
        if get_moved_pos(&position, &direction).y > chars.len() - 1
            || get_moved_pos(&position, &direction).x > chars[0].len() - 1
            || get_moved_pos(&position, &direction).x == 0
            || get_moved_pos(&position, &direction).y == 0
        {
            return false;
        }
        position.move_to(&direction);
        chars[position.y][position.x] = '*';
        if counter > 2000 {
            return true
        }
    }
}

fn get_moved_pos(position: &Position, direction: &Direction) -> Position {
    match direction.direction {
        Up => Position::create(position.x, position.y - 1),
        Left => Position::create(position.x - 1, position.y),
        Down => Position::create(position.x, position.y + 1),
        Right => Position::create(position.x + 1, position.y),
    }
}

impl Direction {
    pub fn turn(&mut self) {
        self.direction = match self.direction {
            Left => Up,
            Up => Right,
            Right => Down,
            Down => Left,
        };
    }
    pub fn create(direction: Directions) -> Direction {
        Direction { direction }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.direction)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

struct Direction {
    direction: Directions,
}
#[derive(Debug)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn get_char(&self, vec: &Vec<Vec<char>>) -> char {
        vec[self.y][self.x]
    }

    pub fn create(x: usize, y: usize) -> Position {
        Position { x, y }
    }
    pub fn move_to(&mut self, direction: &Direction) {
        *self = get_moved_pos(self, direction);
    }
}
