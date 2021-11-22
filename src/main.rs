use push_box::{get_input_map, build_game_status, GameStatus};
use std::collections::VecDeque;


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

        try_extend(&mut bfs_queue, st);
    }
}

fn try_extend(q: &mut VecDeque<GameStatus>, st: GameStatus) {
    todo!()
}
