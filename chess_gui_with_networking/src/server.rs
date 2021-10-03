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
    listening_port: u16,
    port_listener: Option<Box<TcpListener>>,
}

impl Server {
    pub fn new(listening_port: u16) -> Result<Self, String> {
        let listening_port_ip = "127.0.0.1:".to_string() + listening_port.to_string().as_str();
        let port_listener = TcpListener::bind(listening_port_ip.as_str());
        if port_listener.is_err() {
            return Err("Could not bind listening port".to_string());
        }
        //port_listener.as_mut().unwrap().set_nonblocking(true).expect("Cannot set non-blocking");
        println!("server listening on ip: {}:{}", get_local_ip().unwrap(), listening_port);
        let mut chess_game = chess_engine::chess_game::Game::new();
        chess_game.set_up_board();
        Ok(Self {
            spectators: default::Default::default(),
            black_player: default::Default::default(),
            white_player: default::Default::default(),
            chess_game,
            listening_port,
            port_listener: Some(Box::new(port_listener.unwrap())),
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
    pub fn handle_client_request(&mut self) {
        if self.white_player.is_some() {
            // handle white players requests
            //println!("read value");
            let read_value = read_tcp_stream_string(&mut self.white_player.as_mut().unwrap(), 1024);
            if read_value.is_ok() {
                println!("{}", read_value.unwrap());
            }
        }
        if self.black_player.is_some() {
            // handle black players requests
        }
        for _i in self.spectators.iter() {
            // Handle spectator requests
        }
    }
    pub fn send_new_board_state(&mut self) {
        // Update the board state
        // Send out new board state to all clients
    }
}