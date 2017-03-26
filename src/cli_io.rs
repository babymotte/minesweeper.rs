use core::TileState;
use interface::GameHandle;

pub fn print_board(handle: &GameHandle) {

    println!("");
    for y in 0..handle.get_height() {
        for x in 0..handle.get_width() {
            print(handle.get_tile_state(x, y));
        }
        println!("");
    }
    println!("");
}

pub fn print(state: TileState) {
    match state {
        TileState::Uncovered(0) => print!("  "),
        TileState::Uncovered(x) => print!(" {}", x),
        TileState::Covered => print!(" ■"),
        TileState::Detonated => print!(" *"),
        TileState::Marked => print!(" ✓"),
        _ => {}
    }
}