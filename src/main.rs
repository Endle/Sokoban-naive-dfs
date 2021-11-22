use push_box::{get_input_map, build_game_status, GameStatus,print_answer};
use std::collections::VecDeque;
use std::process::exit;


fn main() {
    println!("Hello, world!");
    let ori_graph = get_input_map();
    let ori_status = build_game_status(ori_graph);
    let mut bfs_queue = VecDeque::new();

    bfs_queue.push_back(ori_status);
    loop {
        // println!("ok");
        let g = bfs_queue.pop_front();
        let mut st;
        match  g {
            None => break,
            Some(v) => st = v,
        }

        let r = try_extend(&mut bfs_queue, st);
        match r {
            None => (),
            Some(v) => {
                print_answer(v);
                exit(0);
            },
        }
    }
}



fn try_extend(queue: &mut VecDeque<GameStatus>, st: GameStatus) -> Option<GameStatus>{
    if is_success_state(&st) {
        return Option::Some(st);
    }
    let op = try_extend_up(&st);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_down(&st);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_left(&st);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_right(&st);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    return None;
}

fn is_success_state(st: &GameStatus) -> bool {
    false
}

fn try_extend_up(st: &GameStatus) -> Option<GameStatus> {
    return None;
}

fn try_extend_down(st: &GameStatus) -> Option<GameStatus> {
    return None;
}

fn try_extend_left(st: &GameStatus) -> Option<GameStatus> {
    return None;
}

fn try_extend_right(st: &GameStatus) -> Option<GameStatus> {
    return None;
}

