use std::io::Read;
use std::io::Write;
use std::net::{TcpStream};

use crate::error_handling::chess_gui_error::*;


#[allow(dead_code)]
enum ClientType {
    Player,
    Spectator
}

pub struct Client {
    _client_type: Option<ClientType>,
    server_connection: TcpStream
}

impl Client {
    pub fn new(server_ip: &str) -> ChessGuiResult<Self> {
        let server_connection = Self::connect(server_ip);
        if server_connection.is_err() {
            return Err("Connection to server failed!".to_chess_gui_error());
        }
        Ok(Self {
            _client_type: None,
            server_connection: server_connection.unwrap(),
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
    fn _register_winner(&mut self, _winner: &str) {
        // Winner registered
    }
    fn _draw_request(&mut self, _draw_request: &str) {
        // Opponent asking for draw
    }
    fn _send_draw_request(&mut self) {

    }
    pub fn get_server_input(&mut self) {
        // Fetch server input
        // if it is an update for board state
        self.update_board_state("fen notation");

    }
    pub fn update(&mut self) {
        self.get_server_input();
        //println!("write to tcp stream");
        let _ = write_to_tcp_stream_string(&mut self.server_connection, "hello world!");
        //write_to_tcp_stream(&mut self.server_connection, vec![1, 2]).unwrap();
        // Send requests to server for moves
    }
    fn update_board_state(&mut self, _fen_notation: &str) {
        // Listen for incomming board state
    }
}

pub fn get_local_ip() -> ChessGuiResult<String> {
    use local_ip_address::local_ip;
    let my_local_ip = local_ip();
    if my_local_ip.is_err() {
        return Err("Could not fetch local ip".to_chess_gui_error());
    }
    return Ok(my_local_ip.unwrap().to_string());
}
pub fn read_tcp_stream_bytes(stream: &mut TcpStream, max_read_size: usize) -> ChessGuiResult<Vec<u8>> {
    let mut buf = vec![];
    buf.resize(max_read_size, 0);
    //println!("read");
    match stream.read(&mut buf) {
        Ok(size) => {buf.resize(size, 0)},
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            return Err("socket not ready".to_chess_gui_error());
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
    if buf.len() == 0 {
        return Err("nothing to read".to_chess_gui_error());
    }
    //println!("bytes: {:?}", buf);
    return Ok(buf);
}
pub fn read_tcp_stream_string(stream: &mut TcpStream, max_read_size: usize) -> ChessGuiResult<String> {
    let vec = read_tcp_stream_bytes(stream, max_read_size)?;
    let result = String::from_utf8(vec);
    if result.is_err() {
        return Err("could not convert tcp read to string".to_chess_gui_error());
    }
    return Ok(result.unwrap());
}

pub fn write_to_tcp_stream_bytes(stream: &mut TcpStream, buf: &[u8]) -> ChessGuiResult<()> {
    let result = stream.write(&buf);
    if result.is_err() {
        return Err("Write to tcpstream failed".to_chess_gui_error());
    }
    //println!("write");
    return Ok(());
}

pub fn write_to_tcp_stream_string(stream: &mut TcpStream, buf: &str) -> ChessGuiResult<()> {
    let buf = buf.as_bytes();
    return write_to_tcp_stream_bytes(stream, buf);
}