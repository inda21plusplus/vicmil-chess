use std::net::{TcpStream};
use chess_engine::chess_game::*;

use crate::{error_handling::chess_gui_error::{ChessGuiResult, ToChessGuiError}, networking};

#[allow(dead_code)]
enum ClientType {
    Player,
    Spectator
}

#[allow(dead_code)]
pub struct Client {
    _client_type: Option<ClientType>,
    server_connection: TcpStream,
    pub player_color: ChessPieceColor,
}

impl Client {
    pub fn new(server_ip: &str, player_color: ChessPieceColor) -> ChessGuiResult<Self> {
        let server_connection = Self::connect(server_ip);
        if server_connection.is_err() {
            return Err("Connection to server failed!".to_chess_gui_error());
        }
        Ok(Self {
            _client_type: None,
            server_connection: server_connection.unwrap(),
            player_color
        })
    }
    pub fn connect(server_ip: &str) -> ChessGuiResult<TcpStream> {
        // Connect to server, example 127.0.0.1:1337
        let mut stream = TcpStream::connect(server_ip);
        if stream.is_err() {
            return Err("Connection failed".to_chess_gui_error());
        }
        stream.as_mut().unwrap().set_nonblocking(true).expect("set_nonblocking call failed");
        println!("Client: Succesfully connected to server!");
        return Ok(stream.unwrap());
    }
    pub fn read_from_server(&mut self, chess_game: &mut Game) {
        // Fetch server input
        let result = networking::read_tcp_stream_string(&mut self.server_connection, 1024);
        if result.is_ok() {
            let result = result.unwrap();
            println!("Client: Recieved data from server!: '{}'", result);

            #[derive(Debug, Clone, Copy, PartialEq)]
            enum ReturnType {
                Board,
                End,
            }

            // Parse the string
            'outer: for line in result.split(";") {
                let mut num = 0;
                let mut return_type: Option<ReturnType> = None;
                for arg in line.split(":") {
                    match num {
                        0 => {
                            if arg == "board" {
                                return_type = Some(ReturnType::Board);
                            }
                            else if arg == "end" {
                                return_type = Some(ReturnType::End);
                            }
                            else {
                                // if it is not board, it is not a valid input
                                continue 'outer;
                            }
                        }
                        1 => {
                            if return_type == Some(ReturnType::Board) {
                                println!("Client: setting board from fen notation!: '{}'", arg);
                                // Interprit it as a fen string and update board
                                //let result = chess_game.set_up_board_from_fen(arg);
                                let mut new_game = Game::new();
                                let result = new_game.set_up_board_from_fen(arg.to_string());
                                if result.is_err() {
                                    println!("Client: Failed to set up board: {}", result.err().unwrap());
                                }
                                else {
                                    *chess_game = new_game;
                                    println!("Client: Board setup succesfull!");
                                }
                            }
                            else if return_type == Some(ReturnType::End) {
                                println!("Client: game is over!");
                                let split_arg = arg.split_at(1);
                                //println!("Client: split arg: '{}', '{}'", split_arg.0, split_arg.1);
                                if split_arg.0 == "-" {
                                    println!("Client: It is a Draw!");
                                }
                                else if split_arg.0 == "w" {
                                    println!("Client: White wins!");
                                }
                                else if split_arg.0 == "b" {
                                    println!("Client: Black wins!");
                                }
                                else {
                                    println!("Client: error, cannot parse winner :(");
                                    continue;
                                }

                                println!("Client: setting board from fen notation!: '{}'", arg);
                                let mut new_game = Game::new();
                                let result = new_game.set_up_board_from_fen(split_arg.1.to_string());
                                if result.is_err() {
                                    println!("Client: Failed to set up board: {}", result.err().unwrap());
                                }
                                else {
                                    *chess_game = new_game;
                                    println!("Client: Board setup succesfull!");
                                }
                            }
                        }
                        _ => {
                        }
                    }
                    num = num + 1;
                }
            }
        }
    }
    pub fn update(&mut self, chess_game: &mut Game) {
        self.read_from_server(chess_game);
        //let _ = networking::write_to_tcp_stream_string(&mut self.server_connection, "hello world!");
    }
    pub fn send_move_request(&mut self, board_move: BoardMove, promote_piece: Option<ChessPieceId>, chess_game: &mut Game) -> ChessGuiResult<()> {
        println!("Client: sending move request!");
        // Send a request to server to move piece
        let move_notation: String;
        if chess_game.will_require_promotion(board_move){
            move_notation = move_to_notation(board_move, promote_piece)?;
        }
        else {
            move_notation = move_to_notation(board_move, None)?;
        }
        let move_notation = "move:".to_string() + move_notation.as_str() + ";";
        networking::write_to_tcp_stream_string(&mut self.server_connection, move_notation.as_str())?;
        return Ok(());
    }
}