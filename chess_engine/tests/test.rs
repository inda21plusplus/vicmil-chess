#[cfg(test)]
mod chess_lib_test {
    pub extern crate chess_engine;
    use chess_engine::chess_game::*;
    #[test]
    fn tests_working() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bishop_test() {
        let mut game = Game::new();

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Bishop, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 5, 6);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Bishop, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 6, 6);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Bishop, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 2, 5);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Bishop, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 2, 2);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);
    }
    #[test]
    fn rook_test() {
        let mut game = Game::new();

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Rook, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 5, 6);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Rook, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 3, 6);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        game.empty_board();
        game.set_pos(3, 4, ChessPieceId::Rook, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 6, 4);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);
    }

    #[test]
    fn knight_test() {
        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Knight, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 5, 5);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Knight, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 1, 2);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Knight, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 1, 3);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Knight, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 3, 5);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);
    }

    #[test]
    fn queen_test() {
        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Queen, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 5, 5);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Queen, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 1, 2);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Queen, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 1, 3);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::Queen, ChessPieceColor::White);
        let board_move = BoardMove::new(3, 4, 3, 5);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);
    }

    #[test]
    fn check_test() {
        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::King, ChessPieceColor::White);
        game.set_pos(4, 4, ChessPieceId::Rook, ChessPieceColor::Black);
        assert_eq!(game.is_check().is_some(), true);

        let mut game = Game::new();
        game.set_pos(3, 4, ChessPieceId::King, ChessPieceColor::Black);
        game.set_pos(4, 4, ChessPieceId::Rook, ChessPieceColor::White);
        assert_eq!(game.is_check().is_some(), false); // Since it is white who starts

        // Make sure it is not check when game starts
        let mut game = Game::new();
        game.set_up_board();
        assert_eq!(game.is_check().is_some(), false);
    }

    #[test]
    fn check_stop_move_test() {
        let mut game = Game::new();
        game.set_pos(1, 1, ChessPieceId::Rook, ChessPieceColor::White);
        game.set_pos(3, 4, ChessPieceId::King, ChessPieceColor::White);
        game.set_pos(4, 4, ChessPieceId::Rook, ChessPieceColor::Black);
        let board_move = BoardMove::new(1, 1, 1, 2);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);
    }

    #[test]
    fn pawn_test() {
        let mut game = Game::new();
        game.set_pos(1, 6, ChessPieceId::Pawn, ChessPieceColor::White);
        let board_move = BoardMove::new(1, 6, 1, 5);
        assert_eq!(game.move_piece(board_move, false).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(1, 6, ChessPieceId::Pawn, ChessPieceColor::White);
        let board_move = BoardMove::new(1, 6, 1, 4);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(1, 6, ChessPieceId::Pawn, ChessPieceColor::White);
        let board_move = BoardMove::new(1, 6, 1, 7);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(1, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
        let board_move = BoardMove::new(1, 1, 1, 3);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(1, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
        let board_move = BoardMove::new(1, 1, 1, 2);
        assert_eq!(game.move_piece(board_move, true).is_ok(), true);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(1, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
        let board_move = BoardMove::new(1, 1, 1, 4);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(1, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
        let board_move = BoardMove::new(1, 1, 1, 0);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(2, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
        let board_move = BoardMove::new(2, 1, 3, 7);
        assert_eq!(game.move_piece(board_move, true).is_ok(), false);
    }

    #[test]
    fn convert_test() {
        let mut game = Game::new();
        game.set_pos(0, 0, ChessPieceId::Pawn, ChessPieceColor::White);
        assert_eq!(game.convert(0, 0, ChessPieceId::Pawn).is_ok(), false);

        let mut game = Game::new();
        game.set_pos(0, 0, ChessPieceId::Pawn, ChessPieceColor::White);
        assert_eq!(game.convert(0, 0, ChessPieceId::King).is_ok(), false);

        let mut game = Game::new();
        game.set_pos(0, 0, ChessPieceId::Pawn, ChessPieceColor::White);
        assert_eq!(game.convert(0, 0, ChessPieceId::Queen).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(0, 7, ChessPieceId::Pawn, ChessPieceColor::White);
        assert_eq!(game.convert(0, 7, ChessPieceId::Queen).is_ok(), false);

        let mut game = Game::new();
        game.turn = ChessPieceColor::Black;
        game.set_pos(0, 7, ChessPieceId::Pawn, ChessPieceColor::Black);
        assert_eq!(game.convert(0, 7, ChessPieceId::Queen).is_ok(), true);

        let mut game = Game::new();
        game.set_pos(0, 0, ChessPieceId::Pawn, ChessPieceColor::Black);
        assert_eq!(game.convert(0, 0, ChessPieceId::Queen).is_ok(), false);
    }
}
