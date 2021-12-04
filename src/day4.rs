use std::fs;

pub fn day4() {
    println!("day4");

    let string = fs::read_to_string("bingo.txt").unwrap();
    let mut lines = string.lines();
    const BOARD_SIZE_X: i32 = 5;
    const BOARD_SIZE_Y: i32 = 5;

    let inputs = lines.next().unwrap();
    println!("inputs: {}", inputs);

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
                .map(|s|s.trim())
                .collect();

            for (x, num) in numbers.iter().enumerate() {
                board.add_item(BoardItem::new(x as i32, y, num.parse().unwrap()))
            }
        }
        bingo.boards.push(board);
    }

    println!("game: {:#?}", bingo)
}


#[derive(Debug)]
struct Bingo {
    boards: Vec<Board>,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            boards: vec![]
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Board {
    numbers: Vec<BoardItem>,
    max_x: i32,
    max_y: i32,
}

impl Board {
    fn new(max_x: i32, max_y: i32) -> Board {
        Board {
            numbers: vec![],
            max_x,
            max_y,
        }
    }

    fn add_item(&mut self, item: BoardItem) {
        self.numbers.push(item);
    }
}