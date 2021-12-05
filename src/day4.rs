use std::fs;

use crate::day4::GameState::{Ended, Running};

pub fn day4() {
    println!("day4");

    let string = fs::read_to_string("bingo.txt").unwrap();
    let mut lines = string.lines();
    const BOARD_SIZE_X: i32 = 5;
    const BOARD_SIZE_Y: i32 = 5;

    let inputs: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut bingo = Bingo::new();
    loop {
        let mut board = Board::new(BOARD_SIZE_X, BOARD_SIZE_Y);
        let next = lines.next(); // skip empty line
        if next.is_none() {
            break;
        }

        for y in 0..BOARD_SIZE_Y {
            let line = lines.next().unwrap();
            let numbers: Vec<&str> = line
                .as_bytes()
                .chunks(3)
                .map(std::str::from_utf8)
                .map(|s| s.unwrap())
                .map(|s| s.trim())
                .collect();

            for (x, num) in numbers.iter().enumerate() {
                board.add_item(BoardItem::new(x as i32, y, num.parse().unwrap()))
            }
        }
        bingo.boards.push(board);
    }

    for input in inputs {
        bingo.take_turn(input)
    }

    match bingo.game_state {
        Running => { println!("no winners found") }
        Ended(board, last_input) => { println!("winning game board: {:#?}, with score: {}", board, board.calculate_score(last_input)) }
    }
}


#[derive(Debug)]
struct Bingo {
    boards: Vec<Board>,
    game_state: GameState,
}

#[derive(Debug, PartialEq)]
enum GameState {
    Running,
    Ended(Board, i32),
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            boards: vec![],
            game_state: Running,
        }
    }

    fn take_turn(&mut self, next_number: i32) {
        for mut board in &mut self.boards {
            if self.game_state == Running {
                board.take_turn(next_number);
                if board.has_win_condition {
                    self.game_state = Ended(board.clone(), next_number);
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BoardItem {
    x: i32,
    y: i32,
    value: i32,
    marked: bool,
}

impl BoardItem {
    fn new(x: i32, y: i32, value: i32) -> BoardItem {
        BoardItem { x, y, value, marked: false }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    numbers: Vec<BoardItem>,
    max_x: i32,
    max_y: i32,
    has_win_condition: bool,
}

impl Board {
    fn new(max_x: i32, max_y: i32) -> Board {
        Board {
            numbers: vec![],
            max_x,
            max_y,
            has_win_condition: false,
        }
    }

    fn add_item(&mut self, item: BoardItem) {
        self.numbers.push(item);
    }

    fn take_turn(&mut self, next_number: i32) {
        for mut num in &mut self.numbers {
            if num.value == next_number {
                num.marked = true;
                self.check_win_condition();
                break;
            }
        }
    }

    fn check_win_condition(&mut self) {
        let marked: Vec<BoardItem> = self.numbers
            .iter()
            .filter(|it| it.marked)
            .cloned()
            .collect();

        for x in 0..self.max_x {
            let won: bool = marked
                .iter()
                .filter(|it| it.x == x)
                .cloned()
                .collect::<Vec<BoardItem>>()
                .len()
                .eq(&(self.max_x as usize));

            if won {
                self.has_win_condition = true;
                break;
            }
        }

        for y in 0..self.max_y {
            let won: bool = marked
                .iter()
                .filter(|it| it.y == y)
                .cloned()
                .collect::<Vec<BoardItem>>()
                .len()
                .eq(&(self.max_y as usize));

            if won {
                self.has_win_condition = true;
                break;
            }
        }
    }

    fn calculate_score(&self, last_input: i32) -> i32 {
        return self.numbers
            .iter()
            .filter(|x| !x.marked)
            .map(|x| x.value)
            .sum::<i32>()
            * last_input;
    }
}