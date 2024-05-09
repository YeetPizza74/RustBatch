use std::vec;

fn main() {
    let a: &str = "a";
    let b: &str = "a";
    let c: &str = "a";
    let d: &str = "a";
    let e: &str = "a";
    let f: &str = "a";
    let g: &str = "a";

    set_puzzles(a, b, c, d, e, f, g);
}

fn set_puzzles(scramble: &str, puzzle_def: &str, ignore: &str, subgroups: &str, adjust: &str, post_adjust: &str, sorting: &str) {
    println!("{scramble} {puzzle_def} {ignore} {subgroups} {adjust} {post_adjust} {sorting}");

    let _moves: &str = puzzle_def;
    // let moveLines = ...

    let mut piece_list: Vec<&str> = vec![];
    let mut move_data_list: Vec<&str> = vec![];

    let mut cube_ori: Vec<&str> = vec![];
    let mut move_list: Vec<&str> = vec![];
    let mut clockwise_move_str: Vec<&str> = vec![];

    piece_list.insert(0, "a");
    move_data_list.insert(0, "a");
    cube_ori.insert(0, "a");
    move_list.insert(0, "a");
    clockwise_move_str.insert(0, "a");

    println!("{piece_list:?} {move_data_list:?} {cube_ori:?} {move_list:?} {clockwise_move_str:?}");

    fn test() {
        println!("test");
    }

    test();
}
