use core::{Difficulty, MineField};

#[test]
fn test_new_game_beginner() {
    let minefield = MineField::new(Difficulty::Beginner, 0, 0);
    assert_eq!(minefield.get_width(), 9);
    assert_eq!(minefield.get_height(), 9);
    assert_eq!(minefield.get_mine_count(), 10);
}

#[test]
fn test_new_game_intermediate() {
    let minefield = MineField::new(Difficulty::Intermediate, 0, 0);
    assert_eq!(minefield.get_width(), 16);
    assert_eq!(minefield.get_height(), 16);
    assert_eq!(minefield.get_mine_count(), 40);
}

#[test]
fn test_new_game_expert() {
    let minefield = MineField::new(Difficulty::Expert, 0, 0);
    assert_eq!(minefield.get_width(), 30);
    assert_eq!(minefield.get_height(), 16);
    assert_eq!(minefield.get_mine_count(), 99);
}

#[test]
fn test_new_game_custom() {
    let w = 30;
    let h = 20;
    let m = 120;
    let minefield = MineField::new(Difficulty::Custom(w,h,m), 0, 0);
    assert_eq!(minefield.get_width(), w);
    assert_eq!(minefield.get_height(), h);
    assert_eq!(minefield.get_mine_count(), m);
}