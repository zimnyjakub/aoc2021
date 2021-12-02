#[derive(Debug)]
pub enum Pilot {
    Forward(i32),
    Down(i32),
    Up(i32),
    Nop,
}

impl From<&str> for Pilot {
    fn from(str: &str) -> Self {
        let parts: Vec<&str> = str.split(' ').collect();
        let param = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => Pilot::Forward(param),
            "down" => Pilot::Down(param),
            "up" => Pilot::Up(param),
            &_ => Pilot::Nop
        }
    }
}

#[derive(Debug)]
pub struct Position {
    aim: i32,
    horizontal_position: i32,
    depth: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { aim: 0, horizontal_position: 0, depth: 0 }
    }

    pub fn apply(&mut self, cmds: impl IntoIterator<Item=Pilot>) {
        for cmd in cmds {
            self.apply_one(cmd);
        }
    }

    pub fn apply_one(&mut self, cmd: Pilot) {
        match cmd {
            Pilot::Forward(v) => {
                self.horizontal_position += v;
                self.depth = self.depth + self.aim * v;
            }
            Pilot::Down(v) => { self.aim += v }
            Pilot::Up(v) => { self.aim -= v }
            _ => ()
        }
    }

    pub fn aoc_solution(&self) -> i32 {
        self.depth * self.horizontal_position
    }
}

#[derive(Debug)]
struct SlidingWindow(i32, i32, i32);

impl SlidingWindow {
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_apply_forward() {
        let mut pos = Position::new();

        pos.apply_one("forward 5".into());

        assert_eq!(5, pos.horizontal_position);
        assert_eq!(0, pos.depth);
        assert_eq!(0, pos.aim);
    }

    #[test]
    fn test_apply_forward_down() {
        let mut pos = Position::new();

        pos.apply_one("forward 5".into());
        pos.apply_one("down 5".into());

        assert_eq!(5, pos.horizontal_position);
        assert_eq!(0, pos.depth);
        assert_eq!(5, pos.aim);
    }

    #[test]
    fn test_apply_forward_down_forward() {
        let mut pos = Position::new();

        pos.apply_one("forward 5".into());
        pos.apply_one("down 5".into());
        pos.apply_one("forward 5".into());

        assert_eq!(5+5, pos.horizontal_position);
        assert_eq!(5, pos.aim);
        assert_eq!(5*5, pos.depth);
    }

    #[test]
    fn test_apply_forward_up_forward() {
        let mut pos = Position::new();

        pos.apply_one("forward 5".into());
        pos.apply_one("up 5".into());
        pos.apply_one("forward 5".into());

        assert_eq!(5+5, pos.horizontal_position);
        assert_eq!(-5, pos.aim);
        assert_eq!(-5*5, pos.depth);
    }

    #[test]
    fn test_apply_multiple() {
        let mut pos = Position::new();

        pos.apply_one("forward 5".into());
        pos.apply_one("down 5".into());
        pos.apply_one("forward 8".into());
        pos.apply_one("up 3".into());
        pos.apply_one("down 8".into());
        pos.apply_one("forward 2".into());

        assert_eq!(5+8+2, pos.horizontal_position);
        assert_eq!(5-3+8, pos.aim);
        assert_eq!(60, pos.depth);
    }
}
