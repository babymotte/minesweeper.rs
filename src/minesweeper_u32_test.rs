use minesweeper_u32;
use minesweeper_u32::Action;

#[test]
fn test_get_action() {

    assert_eq!(minesweeper_u32::get_action(0b00000000_00000000_00000000_00000000), Action::StartGame);
    assert_eq!(minesweeper_u32::get_action(0b00111111_11111111_11111111_11111111), Action::StartGame);
    assert_eq!(minesweeper_u32::get_action(0b00001011_11101001_00001001_10101010), Action::StartGame);

    assert_eq!(minesweeper_u32::get_action(0b01000000_00000000_00000000_00000000), Action::UncoverTile);
    assert_eq!(minesweeper_u32::get_action(0b01111111_11111111_11111111_11111111), Action::UncoverTile);
    assert_eq!(minesweeper_u32::get_action(0b01010011_11001001_00010111_10001010), Action::UncoverTile);

    assert_eq!(minesweeper_u32::get_action(0b10000000_00000000_00000000_00000000), Action::ToggleFlag);
    assert_eq!(minesweeper_u32::get_action(0b10111111_11111111_11111111_11111111), Action::ToggleFlag);
    assert_eq!(minesweeper_u32::get_action(0b10010010_00010101_10000101_00010000), Action::ToggleFlag);

    assert_eq!(minesweeper_u32::get_action(0b11000000_00000000_00000000_00000000), Action::NotSpecified);
    assert_eq!(minesweeper_u32::get_action(0b11111111_11111111_11111111_11111111), Action::NotSpecified);
    assert_eq!(minesweeper_u32::get_action(0b11001010_10101010_10011000_11101110), Action::NotSpecified);
}