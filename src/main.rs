use push_box::{get_input_map, build_game_status, GameStatus, print_answer,
               print_map, Direction, try_extend_by_direction};
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

        // print_map(&st.g);
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
    let op = try_extend_by_direction(&st, Direction::Up);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_by_direction(&st, Direction::Down);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_by_direction(&st, Direction::Left);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    let op = try_extend_by_direction(&st, Direction::Right);
    match op {
        None => (),
        Some(v) => queue.push_back(v),
    }
    return None;
}

fn is_success_state(st: &GameStatus) -> bool {
    return st.g.is_success();
}
