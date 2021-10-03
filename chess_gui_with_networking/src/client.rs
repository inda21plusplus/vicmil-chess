use std::net::{TcpStream};
use chess_engine::chess_game::*;

use crate::{parser::move_to_notation};
use crate::networking;

#[allow(dead_code)]
enum ClientType {
    Player,
    Spectator
}

#[allow(dead_code)]
pub struct Client {
    _client_type: Option<ClientType>,
    server_connection: TcpStream,
}

impl Client {
    pub fn new(server_ip: &str) -> Result<Self, String> {
        let server_connection = Self::connect(server_ip);
        if server_connection.is_err() {
            return Err("Connection to server failed!".to_string());
        }
        Ok(Self {
            _client_type: None,
            server_connection: server_connection.unwrap(),
        })
    }
    pub fn connect(server_ip: &str) -> Result<TcpStream, String> {
        // Connect to server, example 127.0.0.1:1337
        let mut stream = TcpStream::connect(server_ip);
        if stream.is_err() {
            return Err("Connection failed".to_string());
        }
        stream.as_mut().unwrap().set_nonblocking(true).expect("set_nonblocking call failed");
        println!("Client: Succesfully connected to server!");
        return Ok(stream.unwrap());
    }
    fn send_draw_request_to_server(&mut self) {

    }
    pub fn read_from_server(&mut self, chess_game: &mut Game) {
        // Fetch server input
        let result = networking::read_tcp_stream_string(&mut self.server_connection, 1024);
        if result.is_ok() {
            // Parse the string
            let result = result.unwrap();
            'outer: for line in result.split(";") {
                let mut num = 0;
                for arg in line.split(":") {
                    match num {
                        0 => {
                            if arg == "board" {

                            }
                            else {
                                // if it is not board, it is not a valid input
                                continue 'outer;
                            }
                        }
                        1 => {
                            // Interprit it as a fen string and update board
                            chess_game.set_up_board_from_fen(arg);
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
    pub fn send_move_request(&mut self, board_move: BoardMove, promote_piece: Option<ChessPieceId>, chess_game: &mut Game) -> Result<(), String> {
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