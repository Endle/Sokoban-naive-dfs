use crate::nd::Arr;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod nd;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum CellStatus {
    Empty,
    Human,
    Box,
    Wall,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

const ROWS: usize = 10;
const COLS: usize = 7;

#[derive(Clone,Copy)]
pub struct Point {
    r:usize,
    c:usize,
}


#[derive(Clone)]
pub struct Graph {
    rows:usize,
    cols:usize,
    cells: Arr<CellStatus>,
    is_target: Arr<bool>,
}

impl Graph {
    pub fn get(&self, p:&Point) -> CellStatus{
        self.cells.get(p.r, p.c)
    }
    pub fn is_success(&self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if !self.satisfied_cell(i, j) {
                    return false;
                }
            }
        }
        true
    }
    fn satisfied_cell(&self, r: usize, c: usize) -> bool {
        if !self.is_target.get(r, c) {
            return true;
        }
        match self.cells.get(r, c) {
            CellStatus::Box => true,
            _ => false
        }
    }
}


#[derive(Clone)]
pub struct GameStatus {
    pub g: Graph,
    path: Vec<Graph>,
    hum: Point,
}

pub fn build_game_status(g:Graph) -> GameStatus {
    let mut path = Vec::new();
    path.push(g.clone());
    let mut hr = 0;
    let mut hc = 0;
    for i in 0..g.rows {
        for j in 0..g.cols {
            if g.cells.get(i,j) == CellStatus::Human {
                hr = i;
                hc = j;
            }
        }
    }
    let mut st = GameStatus{
        g,
        path,
        hum: Point{r: hr, c: hc}
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

fn try_walk(g: &Graph, hum: Point, target: Point) -> Option<Graph> {
    assert!(g.cells.get(hum.r,hum.c)== CellStatus::Human);
    assert!(g.get(&hum)== CellStatus::Human);

    if g.get(&target) != CellStatus::Empty {
        return None;
    }

    let mut newg = g.clone();
    newg.cells.set(hum.r, hum.c, CellStatus::Empty);
    newg.cells.set(target.r, target.c, CellStatus::Human);
    Option::Some(newg)
}


fn try_push_box(g: &Graph, hum: Point, box_cur: Point, box_target: Point) -> Option<Graph>{
    assert!(g.cells.get(hum.r,hum.c)== CellStatus::Human);
    assert!(g.get(&hum)== CellStatus::Human);

    if g.get(&box_cur) != CellStatus::Box {
        return None;
    }
    if g.get(&box_target) != CellStatus::Empty {
        return None;
    }
    let mut newg = g.clone();
    newg.cells.set(hum.r, hum.c, CellStatus::Empty);
    newg.cells.set(box_cur.r, box_cur.c, CellStatus::Human);
    newg.cells.set(box_target.r, box_target.c, CellStatus::Box);

    Option::Some(newg)
}


pub fn try_extend_by_direction(st: &GameStatus, d: Direction) -> Option<GameStatus> {
    let hum = st.hum;

    let next_step = step_by_direction(&st.g,&hum, d);
    if next_step.is_none() {
        return None;
    }
    let next_step = next_step.unwrap();
    let newg = match st.g.get(&next_step) {
        CellStatus::Wall=> None,
        CellStatus::Human=> {
            panic!()
        },
        CellStatus::Empty=> {
            try_walk(&st.g, hum, next_step)
        },
        CellStatus::Box => {
            let box_t = step_by_direction(&st.g, &next_step, d);
            if box_t.is_none() {
                return None;
            }
            try_push_box(&st.g, hum, next_step, box_t.unwrap())
        }
    };


    match newg {
        None => return None,
        Some(v) => {
            let mut new_path = st.path.clone();
            new_path.push(v.clone());
            let mut new_st = GameStatus{
                g: v,
                path: new_path,
                hum: next_step,
            };
            return Option::Some(new_st)
        }
    }
}



// Hack: No need to check the range. We have walls
fn step_by_direction(g: &Graph, p: &Point, d: Direction) -> Option<Point> {
    let mut r = p.clone();

    match d {
        Direction::Up => {
            if p.r == 0 {
                return None
            }
            r.r -= 1;
            Option::Some(r)
        },
        Direction::Down => {
            r.r += 1;
            Option::Some(r)
        },
        Direction::Left => {
            r.c -= 1;
            Option::Some(r)
        },
        Direction::Right => {
            r.c += 1;
            Option::Some(r)
        },
    }

}



