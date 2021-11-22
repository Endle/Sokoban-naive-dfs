use crate::nd::Arr;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod nd;

enum CellStatus {
    Empty,
    Human,
    Box,
}

const ROWS: usize = 10;
const COLS: usize = 7;
pub struct Graph {
    // rows:usize,
    // cols:usize,
    // cells: Arr<CellStatus>,
    // is_target: Arr<bool>,
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

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("Line: {}", line);
    }


    let mut g = Graph{};

    return g;
}

pub fn print_answer(s: GameStatus) {
    return;
}
