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

#[derive(Clone)]
pub struct Graph {
    rows:usize,
    cols:usize,
    cells: Arr<CellStatus>,
    is_target: Arr<bool>,
}


pub struct GameStatus {
    g: Graph,
    path: Vec<Graph>
}

pub fn build_game_status(g:Graph) -> GameStatus {
    let mut path = Vec::new();
    path.push(g.clone());
    let mut st = GameStatus{
        g,
        path
    };
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

    // while r < ROWS {
    //     for c in 0..COLS {
    //         cells.set(r, c, CellStatus::Wall);
    //     }
    //     r += 1;
    // }

    let mut g = Graph{
        rows: r,
        cols: COLS,
        cells,
        is_target : is_tg
    };

    print_map(&g);
    return g;
}

pub fn print_map(g: &Graph) {
    for r in 0..g.rows {
        for c in 0..g.cols {
            match g.cells.get(r, c) {
                CellStatus::Wall => print!("W"),
                CellStatus::Human => print!("P"),
                CellStatus::Empty => {
                    if g.is_target.get(r, c) {
                        print!("T");
                    } else {
                        print!("_");
                    }
                },
                CellStatus::Box => {
                    if g.is_target.get(r, c) {
                        print!("X");
                    } else {
                        print!("B");
                    }
                }
            }
        }
        print!("\n");
    }
}
pub fn print_answer(s: GameStatus) {
    println!("Success!\n");
    return;
}
