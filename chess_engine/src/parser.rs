// OBS! Delar tagna från Vincents kod

pub mod fen_parser {
    use crate::chess_game::*;
    pub const BOARD_SIZE: usize = 8;

    pub const BOARD_X_INPUT: [char; BOARD_SIZE] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    pub const BOARD_Y_INPUT: [char; BOARD_SIZE] = ['8', '7', '6', '5', '4', '3', '2', '1'];

    pub fn get_board(fen_string: String) -> Result<Game, String> {
        let split: Vec<String> = fen_string
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // cant parse, invalid format
        if split.len() != 6 {
            return Err("invalid format!".to_string());
        }

        // get board
        let mut board = Game::new();
        //let mut board = [[EMPTY_PEICE; BOARD_SIZE]; BOARD_SIZE];
        let mut board_x = 0usize;
        let mut board_y = 0usize;
        for char in split[0].chars() {
            if char == '/' || board_x >= BOARD_SIZE {
                board_y += 1;
                board_x = 0;
                continue;
            }
            if board_y >= BOARD_SIZE {
                // this should not happend, invalid input?
                break;
            }

            let piece: Option<ChessPiece>;
            match char {
                'K' => piece = Some(ChessPiece::new(ChessPieceId::King, ChessPieceColor::White)),
                'Q' => piece = Some(ChessPiece::new(ChessPieceId::Queen, ChessPieceColor::White)),
                'N' => piece = Some(ChessPiece::new(ChessPieceId::Knight, ChessPieceColor::White)),
                'B' => piece = Some(ChessPiece::new(ChessPieceId::Bishop, ChessPieceColor::White)),
                'P' => piece = Some(ChessPiece::new(ChessPieceId::Pawn, ChessPieceColor::White)),
                'R' => piece = Some(ChessPiece::new(ChessPieceId::Rook, ChessPieceColor::White)),
                'k' => piece = Some(ChessPiece::new(ChessPieceId::King, ChessPieceColor::Black)),
                'q' => piece = Some(ChessPiece::new(ChessPieceId::Queen, ChessPieceColor::Black)),
                'n' => piece = Some(ChessPiece::new(ChessPieceId::Knight, ChessPieceColor::Black)),
                'b' => piece = Some(ChessPiece::new(ChessPieceId::Bishop, ChessPieceColor::Black)),
                'p' => piece = Some(ChessPiece::new(ChessPieceId::Pawn, ChessPieceColor::Black)),
                'r' => piece = Some(ChessPiece::new(ChessPieceId::Rook, ChessPieceColor::Black)),
                _ => piece = None,
            }
            //let piece = parse_piece(char);

            if piece.is_none() {
                // is number
                let number: Option<u32> = char.to_digit(10);
                if number.is_none() {
                    // invalid input
                    return Err("invalid input!".to_string());
                }
                board_x += number.unwrap() as usize;
            } else {
                let piece = piece.unwrap();
                board.set_pos(BoardPosition::new(board_x as u8, board_y as u8), piece.id, piece.color);
                //board[board_x][board_y] = piece.unwrap();
                board_x += 1;
            }
        }

        // Who to move
        let is_white_to_move = split[1] == "w";
        if is_white_to_move {
            board.turn = ChessPieceColor::White;
        }
        else {
            board.turn = ChessPieceColor::Black;
        }

        // Castle
        let casle_chars: Vec<char> = split[2].chars().collect(); //.chars();

        let mut white_can_castle_king_side = false;
        let mut white_can_castle_queen_side = false;
        let mut black_can_castle_king_side = false;
        let mut black_can_castle_queen_side = false;

        for casle_char in casle_chars {
            match casle_char {
                'K' => {
                    white_can_castle_king_side = true;
                }
                'k' => {
                    black_can_castle_king_side = true;
                }
                'Q' => {
                    white_can_castle_queen_side = true;
                }
                'q' => {
                    black_can_castle_queen_side = true;
                }
                _ => {}
            };
        }

        if !white_can_castle_king_side {
            board.make_unable_white_castle_king_side();
        }
        if !white_can_castle_queen_side {
            board.make_unable_white_castle_queen_side();
        }
        if !black_can_castle_king_side {
            board.make_unable_black_castle_king_side();
        }
        if !black_can_castle_queen_side {
            board.make_unable_black_castle_queen_side();
        }

        let en_passant_position = BoardPosition::from_algebraic_notation(&split[3]);
        if en_passant_position.is_ok() {
            board.last_move_passant = true;
            board.last_move = Some(BoardMove::new(0, 0, 
                en_passant_position.as_ref().unwrap().x, 
            en_passant_position.unwrap().y));
            return Err("En pessant not fully implemented yet!".to_string());
        }

        let half_move_clock = split[4].parse::<u16>();
        if half_move_clock.is_err() {
            // invalid input
            return Err("Half move clock gives error".to_string());
        }

        let full_move_clock = split[5].parse::<u16>();
        if full_move_clock.is_err() {
            // invalid input
            return Err("Full move clock gives error".to_string());
        }

        Ok(board)
    }

    pub fn get_fen_string(game: &mut Game) -> Result<String, String> {
        let mut output = match get_board_fen_string(game) {
            Ok(fen) => fen,
            Err(e) => return Err(e),
        };
        output.push(' ');

        //output += &game.half_move_clock.to_string();
        output += "100";
        output.push(' ');
        output += "50";
        //output += &game.full_move_clock.to_string();

        Ok(output)
    }

    fn get_board_fen_string(game: &mut Game) -> Result<String, String> {
        let mut output: String = String::new();
        // generate board
        for y in 0..BOARD_SIZE {
            let mut last_piece: u8 = 0;
            for x in 0..BOARD_SIZE {
                //let piece_data = game.board[x][y];
                let piece_data = game.get_board_piece_clone(BoardPosition::new(x as u8, y as u8));
                if piece_data.is_none() {
                    last_piece += 1;
                } else {
                    if last_piece != 0 {
                        output.push((last_piece + 48u8) as char); // ascci int to char
                    }
                    match (piece_data.unwrap().id, piece_data.unwrap().color) {
                        (ChessPieceId::King, ChessPieceColor::White) => output.push('K'),
                        (ChessPieceId::Queen, ChessPieceColor::White) => output.push('Q'),
                        (ChessPieceId::Bishop, ChessPieceColor::White) => output.push('B'),
                        (ChessPieceId::Knight, ChessPieceColor::White) => output.push('N'),
                        (ChessPieceId::Pawn, ChessPieceColor::White) => output.push('P'),
                        (ChessPieceId::Rook, ChessPieceColor::White) => output.push('R'),
                        (ChessPieceId::King, ChessPieceColor::Black) => output.push('k'),
                        (ChessPieceId::Queen, ChessPieceColor::Black) => output.push('q'),
                        (ChessPieceId::Bishop, ChessPieceColor::Black) => output.push('b'),
                        (ChessPieceId::Knight, ChessPieceColor::Black) => output.push('n'),
                        (ChessPieceId::Pawn, ChessPieceColor::Black) => output.push('p'),
                        (ChessPieceId::Rook, ChessPieceColor::Black) => output.push('r'),
                    }
                    last_piece = 0
                }
            }
            if last_piece != 0 {
                output.push((last_piece + 48u8) as char);
            }
            if y != BOARD_SIZE - 1 {
                output.push('/');
            }
        }

        // white/black to move
        output.push(' ');
        output.push(if game.turn == ChessPieceColor::White { 'w' } else { 'b' });
        output.push(' ');

        let mut cant_castle = 0;

        // casteling
        //if !game.can_castle_king_side && !game.castle[0].can_castle_queen_side {
        if !game.can_white_castle_king_side() && !game.can_white_castle_queen_side() {
            cant_castle += 1;
        } else {
            if game.can_white_castle_king_side() {
                output.push('K');
            }
            if game.can_white_castle_queen_side() {
                output.push('Q');
            }
        }

        //if !game.castle[1].can_castle_king_side && !game.castle[1].can_castle_queen_side {
        if !game.can_black_castle_king_side() && !game.can_black_castle_queen_side() {
            cant_castle += 1;
        } else {
            if game.can_black_castle_king_side() {
                output.push('k');
            }
            if game.can_black_castle_queen_side() {
                output.push('q');
            }
        }

        if cant_castle == 2 {
            output.push('-');
            output.push(' ');
        }

        if !output.ends_with(' ') {
            output.push(' ');
        }

        if game.last_move_passant {
            let en_passant_position = game.last_move.unwrap();
            output.push(BOARD_X_INPUT[en_passant_position.to_pos.x as usize]);
            output.push(BOARD_Y_INPUT[en_passant_position.to_pos.x as usize]);
        } else {
            output.push('-');
        }

        return Ok(output);
        //return Err("Not implemented yet".to_string());
    }


    pub fn move_to_notation(board_move: BoardMove, promote_piece: Option<ChessPieceId>) -> Result<String, String> {
        let mut return_str = "".to_string();
        return_str += board_move.to_notation()?.as_str();
        if promote_piece.is_some() {
            return_str += promote_piece.unwrap().to_letter().to_uppercase().to_string().as_str();
        }
        else {
            return_str += '-'.to_string().as_str();
        }
        return Ok(return_str);
    }
}