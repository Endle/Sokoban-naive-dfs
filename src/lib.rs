use crate::nd::Arr;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod nd;

#[derive(Copy, Clone)]
enum CellStatus {
    Empty,
    Human,
    Box,
    Wall,
}

const ROWS: usize = 10;
const COLS: usize = 7;
pub struct Graph {
    rows:usize,
    cols:usize,
    cells: Arr<CellStatus>,
    is_target: Arr<bool>,
}

pub struct GameStatus {

}

pub fn build_game_status(g:Graph) -> GameStatus {
    let mut st = GameStatus{};
    return st;
}

pub fn get_input_map() -> Graph {
    let input_file = "game_map.txt";

    let f = File::open(input_file).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut cells = Arr::new((ROWS,COLS), CellStatus::Empty);
    let mut is_tg = Arr::new((ROWS,COLS), false);

    let mut r:usize = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let line = line.as_bytes();

        let mut c:usize = 0;

        while c < line.len() {
            let ch = char::from(line[c]);
            // println!("{}", ch);
            match ch {
                'W' => cells.set(r, c, CellStatus::Wall),
                'T' => is_tg.set(r, c, true),
                'B' => cells.set(r, c, CellStatus::Box),
                'X' => {
                    cells.set(r, c, CellStatus::Box);
                    is_tg.set(r, c, true);
                },
                'O' => (),
                'P' => cells.set(r, c, CellStatus::Human),
                _ => panic!(),
            }
            c+= 1;
        }
        while c < COLS {
            cells.set(r, c, CellStatus::Wall);
            c += 1;
        }

        r += 1;
    }

    while r < ROWS {
        for c in 0..COLS {
            cells.set(r, c, CellStatus::Wall);
        }
        r += 1;
    }

    let mut g = Graph{
        rows: ROWS,
        cols: COLS,
        cells,
        is_target : is_tg
    };

    return g;
}

pub fn print_answer(s: GameStatus) {
    return;
}
