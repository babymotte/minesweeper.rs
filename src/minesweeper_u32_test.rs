use minesweeper_u32;
use minesweeper_u32::Action;
use core::{Difficulty, TileState};
use interface::{GameState, TileUpdate};

/*
 * Don't let yourself be irritated by the unconventional formatting of the binary
 * literals. I've decided to group the digits that encode one piece of information
 * together instead of octets to make the encoding more readable. The actual value
 * of the whole binary number is not of relevance here.
 */

#[test]
fn test_get_action() {

    assert_eq!(minesweeper_u32::get_action(0b_00_00_000000000000_00000000_00000000),
               Action::StartGame);
    assert_eq!(minesweeper_u32::get_action(0b_00_11_111111111111_11111111_11111111),
               Action::StartGame);
    assert_eq!(minesweeper_u32::get_action(0b_00_00_101111101001_00001001_10101010),
               Action::StartGame);

    assert_eq!(minesweeper_u32::get_action(0b_01_00_000000000000_00000000_00000000),
               Action::UncoverTile);
    assert_eq!(minesweeper_u32::get_action(0b_01_11_111111111111_11111111_11111111),
               Action::UncoverTile);
    assert_eq!(minesweeper_u32::get_action(0b_01_01_001111001001_00010111_10001010),
               Action::UncoverTile);

    assert_eq!(minesweeper_u32::get_action(0b_10_00_000000000000_00000000_00000000),
               Action::ToggleFlag);
    assert_eq!(minesweeper_u32::get_action(0b_10_11_111111111111_11111111_11111111),
               Action::ToggleFlag);
    assert_eq!(minesweeper_u32::get_action(0b_10_01_001000010101_10000101_00010000),
               Action::ToggleFlag);

    assert_eq!(minesweeper_u32::get_action(0b_11_00_000000000000_00000000_00000000),
               Action::NotSpecified);
    assert_eq!(minesweeper_u32::get_action(0b_11_11_111111111111_11111111_11111111),
               Action::NotSpecified);
    assert_eq!(minesweeper_u32::get_action(0b_11_00_101010101010_10011000_11101110),
               Action::NotSpecified);
}

#[test]
fn test_get_x() {

    assert_eq!(minesweeper_u32::get_x(0b_00_00_000000000000_00000000_00000000),
               0b_00000000);
    assert_eq!(minesweeper_u32::get_x(0b_00_11_111111111111_11111111_11111111),
               0b_11111111);
    assert_eq!(minesweeper_u32::get_x(0b_00_00_101111101001_00001001_10101010),
               0b_00001001);
    assert_eq!(minesweeper_u32::get_x(0b_01_01_001111001001_00010111_10001010),
               0b_00010111);
    assert_eq!(minesweeper_u32::get_x(0b_10_01_001000010101_10000101_00010000),
               0b_10000101);
    assert_eq!(minesweeper_u32::get_x(0b_11_00_101010101010_10011000_11101110),
               0b_10011000);
}

#[test]
fn test_get_y() {

    assert_eq!(minesweeper_u32::get_y(0b_00_00_000000000000_00000000_00000000),
               0b_00000000);
    assert_eq!(minesweeper_u32::get_y(0b_00_11_111111111111_11111111_11111111),
               0b_11111111);
    assert_eq!(minesweeper_u32::get_y(0b_00_00_101111101001_00001001_10101010),
               0b_10101010);
    assert_eq!(minesweeper_u32::get_y(0b_01_01_001111001001_00010111_10001010),
               0b_10001010);
    assert_eq!(minesweeper_u32::get_y(0b_10_01_001000010101_10000101_00010000),
               0b_00010000);
    assert_eq!(minesweeper_u32::get_y(0b_11_00_101010101010_10011000_11101110),
               0b_11101110);
}

#[test]
fn test_get_mines() {

    assert_eq!(minesweeper_u32::get_mines(0b_00_00_000000000000_00000000_00000000),
               0b_000000000000);
    assert_eq!(minesweeper_u32::get_mines(0b_00_11_111111111111_11111111_11111111),
               0b_111111111111);
    assert_eq!(minesweeper_u32::get_mines(0b_00_00_101111101001_00001001_10101010),
               0b_101111101001);
    assert_eq!(minesweeper_u32::get_mines(0b_01_01_001111001001_00010111_10001010),
               0b_001111001001);
    assert_eq!(minesweeper_u32::get_mines(0b_10_01_001000010101_10000101_00010000),
               0b_001000010101);
    assert_eq!(minesweeper_u32::get_mines(0b_11_00_101010101010_10011000_11101110),
               0b_101010101010);
}

#[test]
fn test_get_difficulty() {

    assert_eq!(minesweeper_u32::get_difficulty(0b_00_00_101111101001_00001001_10101010),
               Difficulty::Beginner);
    assert_eq!(minesweeper_u32::get_difficulty(0b_00_01_001111001001_00010111_10001010),
               Difficulty::Intermediate);
    assert_eq!(minesweeper_u32::get_difficulty(0b_00_10_001000010101_10000101_00010000),
               Difficulty::Expert);
    assert_eq!(minesweeper_u32::get_difficulty(0b_00_11_101010101010_10011000_11101110),
               Difficulty::Custom(0b_10011000, 0b_11101110, 0b_101010101010));
}

#[test]
fn test_convert_game_state_change() {

    assert_eq!(minesweeper_u32::convert_game_state_change(GameState::NotStarted),
               0b_00_00_000000000000_00000000_00000000);

    assert_eq!(minesweeper_u32::convert_game_state_change(GameState::Started),
               0b_00_01_000000000000_00000000_00000000);

    assert_eq!(minesweeper_u32::convert_game_state_change(GameState::Won),
               0b_00_10_000000000000_00000000_00000000);

    assert_eq!(minesweeper_u32::convert_game_state_change(GameState::Lost),
               0b_00_11_000000000000_00000000_00000000);
}

#[test]
fn test_convert_tile_update() {

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_00000000, 0b_00000000, TileState::Covered)),
               0b_01_00_000000000000_00000000_00000000);

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_00000001, 0b_00000001, TileState::Marked)),
               0b_01_01_000000000000_00000001_00000001);

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_00010111, 0b_10010110, TileState::Detonated)),
               0b_01_11_000000000000_00010111_10010110);

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_11111111, 0b_00000000, TileState::Uncovered(0b_0000))),
               0b_01_10_000000000000_11111111_00000000);

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_01010101, 0b_10101010, TileState::Uncovered(0b_0010))),
               0b_01_10_000000000010_01010101_10101010);

    assert_eq!(minesweeper_u32::convert_tile_update(TileUpdate::new(0b_00000111, 0b_00000100, TileState::Uncovered(0b_1010))),
               0b_01_10_000000001010_00000111_00000100);
}