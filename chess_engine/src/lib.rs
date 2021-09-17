pub mod chess_game {
    #[derive(Clone, Copy)]
    pub enum ColorTerminal {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        LightBlue,
        White,
        Count,
        Count2,
        Count3,
        Count4,
    }
    pub fn print_color(text: &str, text_color: ColorTerminal, background_color: ColorTerminal) {
        //let background_mod_8 = background_color as i32 % 8;
        let text_mod_8 = text_color as i32 % 8;
        /*let text_mod;
        if text_color as i32 > 7 {
            text_mod = 1;
        }
        else {
            text_mod = 0;
        }*/

        let background_mod_8 = background_color as i32 % 8;
        /*let background_mod;
        if background_color as i32 > 7 {
            background_mod = 0;
        }
        else {
            background_mod = 1;
        }*/

        print!("\x1b[{}m\x1b[{}m{}\x1b[m", 40+background_mod_8, 30+text_mod_8 as i32, text);
    }
    use std::collections::LinkedList;

    type BoardPos = u8;

    #[derive(Clone, Copy)]
    pub struct BoardMove {
        pub from_x: BoardPos,
        pub from_y: BoardPos,
        pub to_x: BoardPos,
        pub to_y: BoardPos,
    }

    impl BoardMove {
        pub fn new(from_x: BoardPos, from_y: BoardPos, to_x: BoardPos, to_y: BoardPos) -> Self {
            Self {
                from_x,
                from_y,
                to_x,
                to_y,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum ChessPieceId {
        Pawn,
        Knight,
        Rook,
        King,
        Queen,
        Bishop,
    }

    impl ChessPieceId {
        pub fn from_str(text: &str) -> Result<ChessPieceId, ()> {
            let mut text_copy = text.to_string();
            text_copy = text_copy.to_lowercase();
            match text_copy.as_str() {
                "king" => {
                    return Ok(ChessPieceId::King);
                }
                "queen" => {
                    return Ok(ChessPieceId::Queen);
                }
                "rook" => {
                    return Ok(ChessPieceId::Rook);
                }
                "pawn" => {
                    return Ok(ChessPieceId::Pawn);
                }
                "bishop" => {
                    return Ok(ChessPieceId::Bishop);
                }
                "knight" => {
                    return Ok(ChessPieceId::Knight);
                }
                _ => {}
            }
            return Err(());
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum ChessPieceColor {
        White,
        Black,
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct ChessPiece {
        pub id: ChessPieceId,
        pub color: ChessPieceColor,
        pub unicode_char: char,
        pub moved: bool,
    }

    impl ChessPiece {
        pub fn new(id: ChessPieceId, color: ChessPieceColor) -> Self {
            let unicode_char;
            match id {
                ChessPieceId::Bishop => {
                    unicode_char = '♝';
                }
                ChessPieceId::Rook => {
                    unicode_char = '♜';
                }
                ChessPieceId::King => {
                    unicode_char = '♚';
                }
                ChessPieceId::Queen => {
                    unicode_char = '♛';
                }
                ChessPieceId::Knight => {
                    unicode_char = '♞';
                }
                ChessPieceId::Pawn => {
                    unicode_char = '♟';
                }
            }
            Self {
                id: id,
                color: color,
                unicode_char: unicode_char,
                moved: false,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct Game {
        board: [Option<ChessPiece>; 8 * 8],
        pub turn: ChessPieceColor,
        last_move: Option<BoardMove>,
        last_move_passant: bool,
        move_count_left: u32, // To make sure player does not play more than 50 moves
    }

    #[allow(dead_code)]
    impl Game {
        pub fn new() -> Self {
            const INIT: Option<ChessPiece> = None;
            Self {
                board: [INIT; 8 * 8],
                turn: ChessPieceColor::White,
                last_move: None,
                last_move_passant: false,
                move_count_left: 100 // Since it is 50 moves per player
            }
        }
        fn reset_move_count_left(&mut self) {
            self.move_count_left = 100;
        }
        pub fn set_up_board(&mut self) {
            self.empty_board();
            self.set_pos(0, 0, ChessPieceId::Rook, ChessPieceColor::Black);
            self.set_pos(1, 0, ChessPieceId::Knight, ChessPieceColor::Black);
            self.set_pos(2, 0, ChessPieceId::Bishop, ChessPieceColor::Black);
            self.set_pos(3, 0, ChessPieceId::King, ChessPieceColor::Black);
            self.set_pos(4, 0, ChessPieceId::Queen, ChessPieceColor::Black);
            self.set_pos(5, 0, ChessPieceId::Bishop, ChessPieceColor::Black);
            self.set_pos(6, 0, ChessPieceId::Knight, ChessPieceColor::Black);
            self.set_pos(7, 0, ChessPieceId::Rook, ChessPieceColor::Black);
            for x in 0..8 {
                self.set_pos(x, 1, ChessPieceId::Pawn, ChessPieceColor::Black);
            }

            self.set_pos(0, 7, ChessPieceId::Rook, ChessPieceColor::White);
            self.set_pos(1, 7, ChessPieceId::Knight, ChessPieceColor::White);
            self.set_pos(2, 7, ChessPieceId::Bishop, ChessPieceColor::White);
            self.set_pos(3, 7, ChessPieceId::King, ChessPieceColor::White);
            self.set_pos(4, 7, ChessPieceId::Queen, ChessPieceColor::White);
            self.set_pos(5, 7, ChessPieceId::Bishop, ChessPieceColor::White);
            self.set_pos(6, 7, ChessPieceId::Knight, ChessPieceColor::White);
            self.set_pos(7, 7, ChessPieceId::Rook, ChessPieceColor::White);
            for x in 0..8 {
                self.set_pos(x, 6, ChessPieceId::Pawn, ChessPieceColor::White);
            }
        }
        pub fn empty_board(&mut self) {
            for i in 0..self.board.len() {
                self.board[i] = None;
            }
        }
        pub fn print_board(&mut self) {
            println!("  0 1 2 3 4 5 6 7");
            for y in 0..8 {
                print!("{} ", y);
                for x in 0..8 {
                    let background_color;
                    if (x + y) % 2 == 1 {
                        background_color = ColorTerminal::LightBlue;
                    }
                    else {
                        background_color = ColorTerminal::Blue;
                    }
                    let board_ref = *self.get_board_ref(x, y);
                    if board_ref.is_none() {
                        print_color("  ", ColorTerminal::White, background_color);
                    } else {
                        if board_ref.as_ref().unwrap().color == ChessPieceColor::Black {
                            print_color(board_ref.as_ref().unwrap().unicode_char.to_string().as_str(), ColorTerminal::Black, background_color);
                            print_color(" ", ColorTerminal::Black, background_color);
                        }
                        else {
                            print_color(board_ref.as_ref().unwrap().unicode_char.to_string().as_str(), ColorTerminal::White, background_color);
                            print_color(" ", ColorTerminal::White, background_color);
                        }
                    }
                }
                print!("{} ", y);
                println!();
            }
            println!("  0 1 2 3 4 5 6 7");
        }
        pub fn get_board_ref(&mut self, x: BoardPos, y: BoardPos) -> &mut Option<ChessPiece> {
            return &mut self.board[(x + 8 * y) as usize];
        }
        pub fn get_board_piece_clone(&mut self, x: BoardPos, y: BoardPos) -> Option<ChessPiece> {
            return (*self.get_board_ref(x, y)).clone();
        }
        pub fn set_pos(
            &mut self,
            x: BoardPos,
            y: BoardPos,
            id: ChessPieceId,
            color: ChessPieceColor,
        ) {
            *self.get_board_ref(x, y) = Some(ChessPiece::new(id, color));
        }
        pub fn get_possible_moves(&mut self) -> LinkedList<BoardMove> {
            let mut board_moves:LinkedList<BoardMove> = Default::default();
            for x in 0..8 {
                for y in 0..8 {
                    let piece = (self.get_board_ref(x, y)).clone();
                    // If it is a king
                    if piece.is_some() && piece.unwrap().color == self.turn
                    {
                        // Iterate spaces to see if it can move there
                        let mut board_move = BoardMove::new(0, 0, x, y);
                        for x2 in 0..8 {
                            for y2 in 0..8 {
                                board_move.from_x = x2;
                                board_move.from_y = y2;
                                let mut board_copy = self.clone();
                                if board_copy.move_piece(board_move, true).is_ok() {
                                    board_moves.push_back(board_move);
                                }
                            }
                        }
                    }
                }
            }
            return board_moves;
        }
        pub fn is_check(&mut self) -> Option<BoardMove> {
            for x in 0..8 {
                for y in 0..8 {
                    let piece = (self.get_board_ref(x, y)).clone();
                    // If it is a king
                    if piece.is_some() && piece.unwrap().id == ChessPieceId::King && piece.unwrap().color == self.turn
                    {
                        // Iterate pieces to see if any can capture the king
                        let mut board_move = BoardMove::new(0, 0, x, y);
                        for x2 in 0..8 {
                            for y2 in 0..8 {
                                board_move.from_x = x2;
                                board_move.from_y = y2;
                                let mut board_copy = (*self).clone();
                                board_copy.end_turn(); // Make it the opponents turn

                                // Checks if the move captures the king
                                // If it takes the king, you dont have to think about check
                                if board_copy.move_piece(board_move, false).is_ok() {
                                    return Some(board_move);
                                }
                            }
                        }
                    }
                }
            }
            return None;
        }
        fn is_unblocked_straight_line(&mut self, board_move: BoardMove) -> Result<(), String> {
            // Make sure it is a straight line
            if board_move.from_x == board_move.to_x {
                for y in 1..(board_move.from_y as i32 - board_move.to_y as i32).abs() {
                    let mut d_y = y;
                    if board_move.to_y < board_move.from_y {
                        d_y = -d_y;
                    }
                    if self
                        .get_board_ref(board_move.from_x, (d_y + board_move.from_y as i32) as u8)
                        .as_ref()
                        .is_some()
                    {
                        return Err("Straight path is blocked".to_string());
                    }
                }
                return Ok(());
            } else if board_move.from_y == board_move.to_y {
                for x in 1..(board_move.from_x as i32 - board_move.to_x as i32).abs() {
                    let mut d_x = x;
                    if board_move.to_x < board_move.from_x {
                        d_x = -d_x;
                    }
                    if self
                        .get_board_ref((board_move.from_x as i32 + d_x) as u8, board_move.from_y)
                        .as_ref()
                        .is_some()
                    {
                        return Err("Straight path is blocked!".to_string());
                    }
                }
                return Ok(());
            }
            return Err("Not a valid straight line!".to_string());
        }
        fn is_unblocked_diagonal_line(&mut self, board_move: BoardMove) -> Result<(), String> {
            // Make sure it is a diagonal
            if (board_move.from_x as i32 - board_move.to_x as i32).abs()
                == (board_move.from_y as i32 - board_move.to_y as i32).abs()
            {
                // Move along diagonal to make sure path is not blocked
                for i in 1..(board_move.from_x as i32 - board_move.to_x as i32).abs() {
                    let mut d_x = i as i32;
                    let mut d_y = i as i32;
                    if board_move.to_x < board_move.from_x {
                        d_x = -d_x;
                    }
                    if board_move.to_y < board_move.from_y {
                        d_y = -d_y;
                    }
                    if self
                        .get_board_ref(
                            (d_x + board_move.from_x as i32) as u8,
                            (d_y + board_move.from_y as i32) as u8,
                        )
                        .as_ref()
                        .is_some()
                    {
                        return Err("Path blocked!".to_string());
                    }
                }
                return Ok(());
            }
            return Err("Not a valid diagonal!".to_string());
        }

        fn is_move(&mut self, board_move: BoardMove) -> Result<(), String> {
            if (board_move.from_x == board_move.to_x) && (board_move.from_y == board_move.to_y) {
                return Err("Cannot do nothing during your turn!".to_string());
            }
            return Ok(());
        }
        fn is_pieces_same_color(&mut self, x1: BoardPos, y1: BoardPos, x2: BoardPos, y2: BoardPos) -> bool {
            let piece1 = (*self.get_board_ref(x1, y1)).clone();
            let piece2 = (*self.get_board_ref(x2, y2)).clone();
            if piece1.is_some() && piece2.is_some() && piece1.unwrap().color == piece2.unwrap().color {
                return true;
            }
            else {
                return false;
            }
        }
        fn is_piece_id(&mut self, x: BoardPos, y: BoardPos, id: ChessPieceId) -> Result<(), String> {
            if self.get_board_ref(x, y).is_some() && self.get_board_ref(x, y).unwrap().id == id {
                return Ok(());
            }
            else {
                return Err("Piece id does not match".to_string());
            }
        }
        pub fn move_piece(&mut self, board_move: BoardMove, check_for_check: bool) -> Result<(), String> {
            self.is_move(board_move)?;
            self.inside_board(board_move.from_x, board_move.from_y)?;
            self.inside_board(board_move.to_x, board_move.to_y)?;
            let from_piece = self
                .get_board_ref(board_move.from_x, board_move.from_y)
                .clone();
            if from_piece.is_none() {
                return Err("No piece on square selected!".to_string());
            }
            self.board_move_not_same_color_pieces(board_move)?;

            // Make sure move does not lead to check
            if check_for_check {
                let mut self_copy = self.clone();
                self_copy.move_piece(board_move, false)?;
                self_copy.turn = self.turn;
                if self_copy.is_check().is_some() {
                    return Err("Move leads to check!".to_string());
                }
            }

            // make sure you are not moving opponents pieces
            if from_piece.as_ref().unwrap().color != self.turn {
                return Err("Cannot move opponents pieces".to_string());
            }

            // make sure move count limit is not reached
            if self.move_count_left == 0 {
                return Err("Maximum move count reached".to_string());
            }

            // Do move depending on piece
            match from_piece.as_ref().unwrap().id {
                ChessPieceId::Bishop => {
                    self.bishop_move(board_move)?;
                }
                ChessPieceId::Rook => {
                    self.rook_move(board_move)?;
                }
                ChessPieceId::King => {
                    let result1 = self.king_move_one(board_move);
                    let result2 = self.king_castle(board_move);
                    if result1.is_err() && result2.is_err() {
                        return Err(
                            (result1.err().unwrap() + result2.err().unwrap().as_str()).to_string()
                        );
                    }
                }
                ChessPieceId::Queen => {
                    self.queen_move(board_move)?;
                }
                ChessPieceId::Knight => {
                    self.knight_move(board_move)?;
                }
                ChessPieceId::Pawn => {
                    let result1 = self.pawn_one_forward(board_move);
                    let result2 = self.pawn_two_forward(board_move);
                    let result3 = self.pawn_take(board_move);
                    if result1.is_err() && result2.is_err() && result3.is_err() {
                        return Err((result1.err().unwrap()
                            + ", "
                            + result2.err().unwrap().as_str()
                            + ", "
                            + result3.err().unwrap().as_str())
                        .to_string());
                    }
                    // Reset move count after succesfull move with pawn
                    self.reset_move_count_left();
                }
            }
            self.move_count_left = self.move_count_left - 1;
            self.end_turn();
            return Ok(());
        }
        fn inside_board(&mut self, x: BoardPos, y: BoardPos) -> Result<(), String> {
            if x > 7 || y > 7 {
                return Err("Outside of the board!".to_string());
            }
            return Ok(());
        }

        // End the turn and swith 
        fn end_turn(&mut self) {
            if self.turn as u32 == ChessPieceColor::White as u32 {
                self.turn = ChessPieceColor::Black;
            } else {
                self.turn = ChessPieceColor::White;
            }
        }

        // Just moves the piece without any checking
        fn force_move_piece(&mut self, board_move: BoardMove) {
            *self.get_board_ref(board_move.to_x, board_move.to_y) = self
                .get_board_ref(board_move.from_x, board_move.from_y)
                .take();
            if self.get_board_ref(board_move.to_x, board_move.to_y).is_some() {
                self.get_board_ref(board_move.to_x, board_move.to_y).as_mut().unwrap().moved = true;
            }
            // Check for passant
            if self.last_move_passant == true && self.last_move.is_some() {
                if (board_move.to_x - self.last_move.unwrap().from_x) == 0 &&
                    (board_move.to_y as i32 - self.last_move.unwrap().from_y as i32).abs() == 1 &&
                    (board_move.to_y as i32 - self.last_move.unwrap().to_y as i32).abs() == 1 {
                    *self.get_board_ref(self.last_move.unwrap().to_x, self.last_move.unwrap().to_y) =
                        None;
                }
            }
            self.last_move_passant = false;
            self.last_move = Some(board_move);
        }
        fn board_move_not_same_color_pieces(&mut self, board_move: BoardMove) -> Result<(), String> {
            if self.is_pieces_same_color(board_move.from_x, board_move.from_y, board_move.to_x, board_move.to_y) {
                return Err("Cannot move piece to piece of same color".to_string())
            }
            else {
                return Ok(());
            }
        }
        fn board_move_is_forward(&mut self, board_move: BoardMove) -> Result<(), String> {
            let piece = self.get_board_ref(board_move.from_x, board_move.from_y);
            if piece.is_none() {
                return Err("Piece is none".to_string());
            }
            match piece.unwrap().color {
                ChessPieceColor::Black => {
                    if board_move.from_y < board_move.to_y {
                        return Ok(());
                    }
                    else {
                        return Err("Not forward move".to_string());
                    }
                }
                ChessPieceColor::White => {
                    if board_move.from_y > board_move.to_y {
                        return Ok(());
                    }
                    else {
                        return Err("Not forward move".to_string());
                    }
                }
            }
        }

        // Moves
        pub fn convert(
            &mut self,
            x: BoardPos,
            y: BoardPos,
            to_id: ChessPieceId,
        ) -> Result<(), String> {
            self.inside_board(x, y)?;
            if to_id == ChessPieceId::Pawn {
                return Err("Cannot convert to pawn!".to_string());
            }
            if to_id as u32 == ChessPieceId::King as u32 {
                return Err("Cannot convert to king!".to_string());
            }
            let piece = (*self.get_board_ref(x, y)).clone();
            if piece.is_none() {
                return Err("Cannot convert from nothing!".to_string());
            }
            if piece.unwrap().id != ChessPieceId::Pawn {
                return Err("Cannot convert something other than pawn!".to_string());
            }
            if piece.unwrap().color != self.turn {
                return Err("Cannot convert opponents pieces!".to_string());
            }
            // Make sure piece is in the right place
            if (piece.unwrap().color == ChessPieceColor::Black && y == 7)
                || (piece.unwrap().color as u32 == ChessPieceColor::White as u32 && y == 0)
            {
                // Convert piece
                (*self.get_board_ref(x, y)).as_mut().unwrap().id = to_id;
                self.end_turn();
                return Ok(());
            } else {
                return Err("Cannot convert pawn not on other side".to_string());
            }
        }


        fn pawn_one_forward(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Pawn)?;
            self.board_move_is_forward(board_move)?;
            if (board_move.to_y as i32 - board_move.from_y as i32).abs() != 1 {
                return Err("Not moving one forward".to_string());
            }
            if (board_move.to_x as i32 - board_move.from_x as i32).abs() != 0 {
                return Err("Moving to side".to_string());
            }
            if self
                .get_board_ref(board_move.to_x, board_move.to_y)
                .is_some()
            {
                return Err("Path blocked".to_string());
            }

            self.force_move_piece(board_move);
            return Ok(());
        }

        fn pawn_two_forward(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Pawn)?;
            self.board_move_is_forward(board_move)?;
            self.is_unblocked_straight_line(board_move)?;

            // Make sure piece is moving two forward
            if (board_move.to_y as i32 - board_move.from_y as i32).abs() != 2 {
                return Err("Not moving two forward".to_string());
            }
            if (board_move.to_x as i32 - board_move.from_x as i32).abs() != 0 {
                return Err("Moving to side".to_string());
            }

            if self
                .get_board_ref(board_move.to_x, board_move.to_y)
                .is_some()
            {
                return Err("Path blocked".to_string());
            }

            if self.get_board_ref(board_move.from_x, board_move.from_y).unwrap().moved == true {
                return Err("Cannot move pawn two forward who has already moved".to_string());
            }

            // Move piece
            self.force_move_piece(board_move);
            self.last_move_passant = true;
            
            return Ok(());
        }

        fn pawn_take(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Pawn)?;
            self.board_move_not_same_color_pieces(board_move)?;
            self.board_move_is_forward(board_move)?;

            // Make sure pawn is moving one forward
            if (board_move.to_y as i32 - board_move.from_y as i32).abs() != 1 {
                return Err("Not moving one forward".to_string());
            }

            // Make sure pawn is moving one to the side
            if (board_move.to_x as i32 - board_move.from_x as i32).abs() != 1 {
                return Err("Not moving one to the side".to_string());
            }

            // Make sure there is a piece to take
            if self
                .get_board_ref(board_move.to_x, board_move.to_y)
                .is_none()
            {
                return Err("No piece to take".to_string());
            }

            // Move piece
            self.force_move_piece(board_move);
            return Ok(());
        }

        fn rook_move(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Rook)?;
            self.is_unblocked_straight_line(board_move)?;
            self.board_move_not_same_color_pieces(board_move)?;

            self.force_move_piece(board_move);
            return Ok(());
        }

        fn knight_move(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Knight)?;
            self.board_move_not_same_color_pieces(board_move)?;

            if ((board_move.from_x as i32 - board_move.to_x as i32).abs() != 1
            || (board_move.from_y as i32 - board_move.to_y as i32).abs() != 2)
                && ((board_move.from_x as i32 - board_move.to_x as i32).abs() != 2
                || (board_move.from_y as i32 - board_move.to_y as i32).abs() != 1) {
                return Err("Knight: invalid move".to_string());
            }
            self.force_move_piece(board_move);
            return Ok(());
        }

        fn queen_move(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::Queen)?;
            self.board_move_not_same_color_pieces(board_move)?;
            let diagonal_result = self.is_unblocked_diagonal_line(board_move);
            let straight_result = self.is_unblocked_straight_line(board_move);
            if diagonal_result.is_ok() || straight_result.is_ok() {
                return Ok(());
            } else {
                let mut error_message: String = "Queen: ".to_string();
                error_message += diagonal_result.err().unwrap().as_str();
                error_message += ", ";
                error_message += straight_result.err().unwrap().as_str();
                return Err(error_message);
            }
        }

        fn king_move_one(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_piece_id(board_move.from_x, board_move.from_y, ChessPieceId::King)?;
            self.board_move_not_same_color_pieces(board_move)?;
            if (board_move.from_x as i32 - board_move.to_x as i32).abs() > 1
                || (board_move.from_y as i32 - board_move.to_y as i32).abs() > 1
            {
                return Err("King: cannot move that far!".to_string());
            } else {
                return Ok(());
            }
        }

        fn king_castle(&mut self, board_move: BoardMove) -> Result<(), String> {
            let from_piece = self.get_board_piece_clone(board_move.from_x, board_move.from_y);
            if from_piece.is_none() || (from_piece.unwrap().id != ChessPieceId::King) {
                return Err("Must have king to castle".to_string());
            }
            if from_piece.unwrap().moved == true {
                return Err("Cannot castle with moved piece".to_string());
            }
            if (board_move.to_x as i32- board_move.from_x as i32).abs() != 2 {
                return Err("Castle requires piece to move two to the side".to_string());
            }
            if (board_move.to_y as i32- board_move.from_y as i32).abs() != 0 {
                return Err("Cannot move forwards while casteling".to_string());
            }
            // Find the rook
            let direction: i32;
            if board_move.to_x > board_move.from_x {
                direction = 1;
            }
            else {
                direction = -1;
            }
            for i in 1..3 {
                let rook_x = (board_move.to_x as i32+i*direction) as BoardPos;
                let rook_y = board_move.from_y;
                if self.is_piece_id(rook_x, rook_y, ChessPieceId::Rook).is_ok() {
                    if self.get_board_ref(rook_x, rook_y).unwrap().moved == true {
                        return Err("Cannot castle with moved rook".to_string());
                    }
                    // Make sure it is not chess on any of the squares


                    // Move king and rook
                    self.force_move_piece(board_move);
                    let rook_move = BoardMove::new(rook_x, rook_y, 
                        (board_move.from_x as i32 + direction) as BoardPos, rook_y);
                    self.force_move_piece(rook_move);
                    return Ok(());
                }
            }
            return Err("No rook to move with".to_string());
        }

        fn bishop_move(&mut self, board_move: BoardMove) -> Result<(), String> {
            self.is_unblocked_diagonal_line(board_move)?;
            self.force_move_piece(board_move);
            return Ok(());
        }
    }
}
