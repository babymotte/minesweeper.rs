use core::{Difficulty, MineField};

#[test]
fn test_new_game_beginner() {
    let minefield = MineField::new(Difficulty::Beginner, 0, 0);
    assert_eq!(minefield.get_width(), 9);
    assert_eq!(minefield.get_height(), 9);
}

#[test]
fn test_new_game_intermediate() {
    let minefield = MineField::new(Difficulty::Intermediate, 0, 0);
    assert_eq!(minefield.get_width(), 16);
    assert_eq!(minefield.get_height(), 16);
}

#[test]
fn test_new_game_expert() {
    let minefield = MineField::new(Difficulty::Expert, 0, 0);
    assert_eq!(minefield.get_width(), 30);
    assert_eq!(minefield.get_height(), 16);
}