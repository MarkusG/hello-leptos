#[cfg(test)]
#[rustfmt::skip]
pub mod tests {
use leptos::prelude::Get;
use reactive_stores::Store;
use crate::components::game::board_state::{BoardState, Playable};

    #[test]
    fn board_iter_test() {
        let board_state = BoardState::from_tiles(
            vec![
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );

        assert_eq!(
            board_state.iter().collect::<Vec<_>>(),
            vec![
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn board_move_merge_test() {
        let board_state = BoardState::from_tiles(
            vec![
                0, 0, 2, 2,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );

        let board = Store::new(board_state);

        board.move_right();

        assert_eq!(
            board.get().iter().collect::<Vec<_>>(),
            vec![
                0, 0, 0, 4,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn board_move_pack_test() {
        let board_state = BoardState::from_tiles(
            vec![
                0, 2, 4, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );

        let board = Store::new(board_state);

        board.move_right();

        assert_eq!(
            board.get().iter().collect::<Vec<_>>(),
            vec![
                0, 0, 2, 4,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn board_move_pack_and_merge_test() {
        let board_state = BoardState::from_tiles(
            vec![
                0, 2, 2, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );

        let board = Store::new(board_state);

        board.move_right();

        assert_eq!(
            board.get().iter().collect::<Vec<_>>(),
            vec![
                0, 0, 0, 4,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn board_move_merge_precedence_test() {
        let board_state = BoardState::from_tiles(
            vec![
                0, 2, 2, 2,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );

        let board = Store::new(board_state);

        board.move_right();

        assert_eq!(
            board.get().iter().collect::<Vec<_>>(),
            vec![
                0, 0, 2, 4,
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0
            ]
        )
    }
}
