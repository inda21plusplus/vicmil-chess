use chess_engine::chess_game::ChessPieceColor;

use crate::networking::*;
use std::net::{TcpListener, TcpStream};
use std::collections::LinkedList;
use std::default;

#[allow(dead_code)]
pub struct Server {
    black_player: Option<TcpStream>,
    white_player: Option<TcpStream>,
    spectators: LinkedList<TcpStream>,
    chess_game: chess_engine::chess_game::Game,
    pub listening_port: u16,
    port_listener: Option<Box<TcpListener>>,
    pub server_ip: String,
}

impl Server {
    pub fn new(listening_port: u16, local: bool) -> Result<Self, String> {
        let listening_port_ip: String;
        if local {
            listening_port_ip = "127.0.0.1:".to_string() + listening_port.to_string().as_str();
        }
        else {
            listening_port_ip = get_local_ip().unwrap() + ":" + listening_port.to_string().as_str();
        }
        let port_listener = TcpListener::bind(listening_port_ip.as_str());
        if port_listener.is_err() {
            return Err("Could not bind listening port: ".to_string() + listening_port_ip.as_str());
        }
        //port_listener.as_mut().unwrap().set_nonblocking(true).expect("Cannot set non-blocking");
        //println!("server listening on ip: {}:{}", get_local_ip().unwrap(), listening_port);
        println!("server listening on ip: {}", listening_port_ip);
        let mut chess_game = chess_engine::chess_game::Game::new();
        chess_game.set_up_board();
        Ok(Self {
            spectators: default::Default::default(),
            black_player: default::Default::default(),
            white_player: default::Default::default(),
            chess_game,
            listening_port,
            port_listener: Some(Box::new(port_listener.unwrap())),
            server_ip: listening_port_ip,
        })
    }
    fn handle_client(&mut self, stream: TcpStream) {
        stream.set_nonblocking(true).expect("set_nonblocking call failed");
        if self.white_player.is_none() {
            println!("Server: Add white player");
            self.white_player = Some(stream);
            return;
        }
        if self.black_player.is_none() {
            println!("Server: Add black player");
            self.black_player = Some(stream);
            // Start game
            return;
        }
        println!("Server: Add spectator");
        self.spectators.push_back(stream);
    }
    pub fn accept_incomming_connections(&mut self) -> Result<(), String> {
        let mut port_listener = self.port_listener.take();
        let mut error: Option<String> = None;
        let _ = port_listener.as_mut().unwrap().set_nonblocking(true);
        // accept connections and process them serially
        for stream in port_listener.as_mut().unwrap().incoming() {
            //println!("Server: Incomming connection!");
            if stream.is_err() {
                error = Some("stream was error".to_string());
                break;
            }
            self.handle_client(stream.unwrap());
        }
        self.port_listener = port_listener;
        if error.is_some() {
            return Err(error.unwrap());
        }
        return Ok(());
    }
    // Take requests from clients, update board state
    pub fn update(&mut self) {
        //println!("update");
        let _ = self.accept_incomming_connections();
        self.handle_client_request();
    }
    pub fn parse_client_input(&mut self, input_string: String, client_color: Option<chess_engine::chess_game::ChessPieceColor>) {
        // Parse the string
        #[derive(Clone, Copy)]
        enum RequestType {
            Move,
            None,
        }
        let mut request_type: RequestType = RequestType::None;
        'outer: for line in input_string.split(";") {
            println!("Server: recieved request: '{}'", line);
            let mut num = 0;
            for arg in line.split(":") {
                match num {
                    0 => {
                        if arg == "move" {
                            request_type = RequestType::Move;
                        }
                        else {
                            // if it is not board, it is not a valid input
                            continue 'outer;
                        }
                    }
                    1 => {
                        if request_type as usize == RequestType::Move as usize {
                            if Some(self.chess_game.turn) == client_color {
                                let mut arg = arg.to_string();
                                if arg.len() < 5 {
                                    // if it is not the right size, it is not a valid input
                                    continue 'outer;
                                }
                                // Make promotion char into uppercase
                                if arg.as_bytes()[4] == '-' as u8 {
                                }
                                else if (arg.as_bytes()[4] as char).is_alphabetic() {
                                    let arg_split = arg.split_at(3);
                                    arg = arg_split.0.to_string() + arg_split.1.to_string().to_uppercase().as_str();
                                }

                                // Try performing move
                                let result = self.chess_game.algebraic_notation_move(arg.to_string());
                                if result.is_ok() {
                                    self.send_new_board_state();
                                    println!("Server: Move succesfull");
                                }
                                else {
                                    println!("Server: Move failed, '{}' {}", arg, result.err().unwrap());
                                }
                            }
                            else {
                                println!("Server: Move failed, Wrong player color");
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
    pub fn handle_client_request(&mut self) {
        if self.white_player.is_some() {
            // handle white players requests
            let read_value = read_tcp_stream_string(&mut self.white_player.as_mut().unwrap(), 1024);
            if read_value.is_ok() {
                let read_value = read_value.unwrap();
                println!("{}", read_value);
                self.parse_client_input(read_value, Some(ChessPieceColor::White));
            }
        }
        if self.black_player.is_some() {
            // handle black players requests
            let read_value = read_tcp_stream_string(&mut self.black_player.as_mut().unwrap(), 1024);
            if read_value.is_ok() {
                let read_value = read_value.unwrap();
                println!("{}", read_value);
                self.parse_client_input(read_value, Some(ChessPieceColor::Black));
            }
        }
        for _i in self.spectators.iter() {
            // Handle spectator requests
        }
    }
    pub fn send_new_board_state(&mut self) {
        // Send out new board state to all clients
        let fen_notation = self.chess_game.get_fen();
        if fen_notation.is_err() {
            panic!("Conversion to fen notation failed!");
        }
        let send_msg;
        // Check if game is over
        if self.chess_game.game_is_over() {
            let winner = self.chess_game.get_winner();
            if winner.is_none() {
                send_msg = "end:-".to_string() + fen_notation.unwrap().as_str() + ";";
            }
            else if winner.unwrap() == ChessPieceColor::White {
                send_msg = "end:w".to_string() + fen_notation.unwrap().as_str() + ";";
            }
            else {
                send_msg = "end:b".to_string() + fen_notation.unwrap().as_str() + ";";
            }
        }
        else {
            send_msg = "board:".to_string() + fen_notation.unwrap().as_str() + ";";
        }
        if self.white_player.is_some() {
            let _ = crate::networking::write_to_tcp_stream_string(self.white_player.as_mut().unwrap(), send_msg.as_str());
        }
        if self.black_player.is_some() {
            let _ = crate::networking::write_to_tcp_stream_string(self.black_player.as_mut().unwrap(), send_msg.as_str());
        }
        for spectator in self.spectators.iter_mut() {
            let _ = crate::networking::write_to_tcp_stream_string(spectator, send_msg.as_str());
        }
    }

}