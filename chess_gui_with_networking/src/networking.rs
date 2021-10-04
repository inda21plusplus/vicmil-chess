use std::io::Read;
use std::io::Write;
use std::net::{TcpStream};


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

/*struct Game {
    client: Option<Client>,
    server: Option<Server>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            client: None,
            server: None
        }
    }
    // Update game state, both server and client
    pub fn update(&mut self) {
        if self.server.is_some() {
            self.server.as_mut().unwrap().update();
        }
        if self.client.is_some() {
            self.client.as_mut().unwrap().update();
        }
    }

    pub fn host(&mut self) {
        // Crate a server
        self.server = Some(Server::new(4828).unwrap());

        // Connect a client to the server
        self.client = Some(Client::new("127.0.0.1:4828").unwrap());
    }

    #[allow(dead_code)]
    pub fn connect(&mut self, server_ip: &str) {
        // Set the server to none
        self.server = None;

        // Connect a client to an external server
        self.client = Some(Client::new(server_ip).unwrap());
    }
}*/

pub fn get_local_ip() -> Result<String, String> {
    use local_ip_address::local_ip;
    let my_local_ip = local_ip();
    if my_local_ip.is_err() {
        return Err("Could not fetch local ip".to_string());
    }
    return Ok(my_local_ip.unwrap().to_string());
}
pub fn read_tcp_stream_bytes(stream: &mut TcpStream, max_read_size: usize) -> Result<Vec<u8>, String> {
    let mut buf = vec![];
    buf.resize(max_read_size, 0);
    //println!("read");
    match stream.read(&mut buf) {
        Ok(size) => {buf.resize(size, 0)},
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            return Err("socket not ready".to_string());
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
    if buf.len() == 0 {
        return Err("nothing to read".to_string());
    }
    //println!("bytes: {:?}", buf);
    return Ok(buf);
}
pub fn read_tcp_stream_string(stream: &mut TcpStream, max_read_size: usize) -> Result<String, String> {
    let vec = read_tcp_stream_bytes(stream, max_read_size)?;
    let result = String::from_utf8(vec);
    if result.is_err() {
        return Err("could not convert tcp read to string".to_string());
    }
    return Ok(result.unwrap());
}

pub fn write_to_tcp_stream_bytes(stream: &mut TcpStream, buf: &[u8]) -> Result<(), String> {
    let result = stream.write(&buf);
    if result.is_err() {
        return Err("Write to tcpstream failed".to_string());
    }
    //println!("write");
    return Ok(());
}

pub fn write_to_tcp_stream_string(stream: &mut TcpStream, buf: &str) -> Result<(), String> {
    let buf = buf.as_bytes();
    return write_to_tcp_stream_bytes(stream, buf);
}

/*fn main() {
    println!("Creating game..");
    let mut game = Game::new();
    println!("Setting up game host..");
    game.host();
    println!("Running game..");
    loop {
        game.update();
    }
}*/