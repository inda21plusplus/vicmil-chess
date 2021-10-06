use std::io::{self, BufRead};

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;
use chess_engine::chess_game::*;
use crate::server::*;
use crate::client::*;

pub mod client;
pub mod server;
pub mod networking;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

        ggez::graphics::set_window_title(&ctx, "Chess Graphical Interface");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx).unwrap();
    my_game.game.set_up_board();
    

    println!("game setup type: 'local', 'broadcast', 'host', 'join <IP_string>'");
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let user_input: Vec<String> =
            line.split_whitespace().map(|num| num.to_string()).collect();
        match user_input[0].as_str() {
            "local" => {
                let result = my_game.stop_hosting();
                if result.is_ok() {
                    println!("succesfully set up local game");
                    break;
                }
                else {
                    println!("local game setup failed: {}", result.err().unwrap());
                }
            },
            "host" => {
                let result = my_game.host_server(1337, false);
                if result.is_ok() {
                    println!("succesfully set up server");
                    break;
                }
                else {
                    println!("server setup failed: {}", result.err().unwrap());
                }
            },
            "local_host" => {
                let result = my_game.host_server(1337, true);
                if result.is_ok() {
                    println!("succesfully set up server");
                    break;
                }
                else {
                    println!("server setup failed: {}", result.err().unwrap());
                }
            },
            "join" => {
                if user_input.len() > 1 {
                    let server_ip = user_input[1].clone();
                    let result = my_game.connect_to_server(server_ip);
                    if result.is_ok() {
                        println!("succesfully connected to server");
                        break;
                    }
                    else {
                        println!("connection to server failed: {}", result.err().unwrap());
                    }
                }
            },
            "broadcast" => {
                let result = my_game.host_allow_spectators(1337);
                if result.is_ok() {
                    println!("succesfully set up broadcast server");
                    break;
                }
                else {
                    println!("server setup failed: {}", result.err().unwrap());
                }
            },
            _ => {}
        }
    }
    

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    black_rook: graphics::Image,
    white_rook: graphics::Image,
    black_queen: graphics::Image,
    white_queen: graphics::Image,
    black_pawn: graphics::Image,
    white_pawn: graphics::Image,
    black_bishop: graphics::Image,
    white_bishop: graphics::Image,
    black_knight: graphics::Image,
    white_knight: graphics::Image,
    black_king: graphics::Image,
    white_king: graphics::Image,
    black_square: graphics::Image,
    white_square: graphics::Image,
    red_square: graphics::Image,
    wait_for_your_turn: graphics::Image,
    mouse_button_press_down: Option<ggez::mint::Point2<f32>>,
    game: chess_engine::chess_game::Game,
    client1: Option<Client>,
    client2: Option<Client>,
    server: Option<Server>,
}

pub fn get_square_from_mouse_pos(pos: ggez::mint::Point2<f32>) -> Result<ggez::mint::Point2<u8>, String> {
    if pos.x > 0.0 && pos.y > 0.0 && pos.x < SCREEN_WIDTH && pos.y < SCREEN_HEIGHT {
        return Ok(ggez::mint::Point2{
            x: ((pos.x*8.0) / SCREEN_WIDTH) as u8,
            y: ((pos.y*8.0) / SCREEN_HEIGHT) as u8,
        })
    }
    return Err("Outside bounds".to_string());
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let black_rook = graphics::Image::new(ctx, "/Chess_rdt60.png")?;
        let white_rook = graphics::Image::new(ctx, "/Chess_rlt60.png")?;
        let black_queen = graphics::Image::new(ctx, "/Chess_qdt60.png")?;
        let white_queen = graphics::Image::new(ctx, "/Chess_qlt60.png")?;
        let black_pawn = graphics::Image::new(ctx, "/Chess_pdt60.png")?;
        let white_pawn = graphics::Image::new(ctx, "/Chess_plt60.png")?;
        let black_bishop = graphics::Image::new(ctx, "/Chess_bdt60.png")?;
        let white_bishop = graphics::Image::new(ctx, "/Chess_blt60.png")?;
        let black_knight = graphics::Image::new(ctx, "/Chess_ndt60.png")?;
        let white_knight = graphics::Image::new(ctx, "/Chess_nlt60.png")?;
        let black_king = graphics::Image::new(ctx, "/Chess_kdt60.png")?;
        let white_king = graphics::Image::new(ctx, "/Chess_klt60.png")?;
        let black_square = graphics::Image::new(ctx, "/black_square.png")?;
        let white_square = graphics::Image::new(ctx, "/white_square.png")?;
        let red_square = graphics::Image::new(ctx, "/red_square.png")?;
        let wait_for_your_turn = graphics::Image::new(ctx, "/wait_for_your_turn.png")?;

        let game = chess_engine::chess_game::Game::new();

        let s = MyGame {
            black_rook,
            white_rook,
            black_queen,
            white_queen,
            black_pawn,
            white_pawn,
            black_bishop,
            white_bishop,
            black_knight,
            white_knight,
            black_king,
            white_king,
            black_square,
            white_square,
            wait_for_your_turn,
            red_square,
            mouse_button_press_down: None,
            game,
            client1: None,
            client2: None,
            server: None
        };

        Ok(s)
    }


    #[allow(dead_code)]
    pub fn host_server(&mut self, port: u16, local: bool) -> Result<(), String> {
        self.server = Some(Server::new(port, local)?);
        let server_ip: String = self.server.as_mut().unwrap().server_ip.clone();
        let client = Client::new(server_ip.as_str(), ChessPieceColor::White);
        if client.is_err() {
            self.server = None;
            self.client1 = None;
            self.client2 = None;
        }
        self.client1 = Some(client.unwrap());
        self.client2 = None;
        return Ok(());
    }

    #[allow(dead_code)]
    pub fn connect_to_server(&mut self, server_ip: String) -> Result<(), String> {
        let client = Client::new(server_ip.as_str(), ChessPieceColor::Black);
        if client.is_err() {
            self.server = None;
            self.client1 = None;
            self.client2 = None;
            return Err("Could not connect to server!".to_string());
        }
        self.server = None;
        self.client1 = Some(client.unwrap());
        self.client2 = None;
        return Ok(());
    }

    #[allow(dead_code)]
    pub fn host_allow_spectators(&mut self, port: u16) -> Result<(), String> {
        self.server = Some(Server::new(port, false)?);
        let server_ip = self.server.as_mut().unwrap().server_ip.clone();
        let client1 = Client::new(server_ip.as_str(), ChessPieceColor::White);
        let client2 = Client::new(server_ip.as_str(), ChessPieceColor::Black);
        if client1.is_err() || client2.is_err() {
            self.server = None;
            self.client1 = None;
            self.client2 = None;
        }
        self.client1 = Some(client1.unwrap());
        self.client2 = Some(client2.unwrap());
        return Ok(());
    }

    #[allow(dead_code)]
    pub fn stop_hosting(&mut self) -> Result<(), String> {
        self.server = None;
        self.client1 = None;
        self.client2 = None;
        return Ok(());
    }

    pub fn get_board_piece_image(&mut self, chess_piece: ChessPiece) -> Option<&graphics::Image> {
        let image: Option<&graphics::Image>;
        match (chess_piece.color, chess_piece.id) {
            (ChessPieceColor::Black, ChessPieceId::King) => image = Some(&self.black_king),
            (ChessPieceColor::Black, ChessPieceId::Queen) => image = Some(&self.black_queen),
            (ChessPieceColor::Black, ChessPieceId::Rook) => image = Some(&self.black_rook),
            (ChessPieceColor::Black, ChessPieceId::Bishop) => image = Some(&self.black_bishop),
            (ChessPieceColor::Black, ChessPieceId::Knight) => image = Some(&self.black_knight),
            (ChessPieceColor::Black, ChessPieceId::Pawn) => image = Some(&self.black_pawn),
            (ChessPieceColor::White, ChessPieceId::King) => image = Some(&self.white_king),
            (ChessPieceColor::White, ChessPieceId::Queen) => image = Some(&self.white_queen),
            (ChessPieceColor::White, ChessPieceId::Rook) => image = Some(&self.white_rook),
            (ChessPieceColor::White, ChessPieceId::Bishop) => image = Some(&self.white_bishop),
            (ChessPieceColor::White, ChessPieceId::Knight) => image = Some(&self.white_knight),
            (ChessPieceColor::White, ChessPieceId::Pawn) => image = Some(&self.white_pawn)
        }
        return image;
    }

    pub fn draw_chess_board(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut grabbed_piece_pos: Option<ggez::mint::Point2<u8>> = None;
        let mut grabbed_piece: Option<ChessPiece> = None;
        if ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) 
        || ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Right) {
            let mouse_down_board_pos = get_square_from_mouse_pos(self.mouse_button_press_down.unwrap());
            if mouse_down_board_pos.is_ok() {
                grabbed_piece_pos = Some(mouse_down_board_pos.unwrap());
            }
        }

        for x in 0..8 {
            for y in 0..8 {
                // Draw black and white squares
                {
                    let image = &self.black_square;
                    let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
                    let scale = glam::Vec2::new(scale_factor, scale_factor);
                    let size = image.dimensions().h * scale.x;
                    let dst = glam::Vec2::new(size*x as f32, size*y as f32);

                    // Color square red if player can move there
                    let mut board_copy = self.game.clone();
                    if grabbed_piece_pos.is_some() 
                        && board_copy.move_piece(BoardMove::new(grabbed_piece_pos.as_mut().unwrap().x, 
                        grabbed_piece_pos.as_mut().unwrap().y, 
                        x, 
                        y), 
                        true, Some(ChessPieceId::Queen)).is_ok() {
                            let image = &self.black_square;
                            let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
                            let scale = glam::Vec2::new(scale_factor, scale_factor);
                            let size = image.dimensions().h * scale.x;
                            let dst = glam::Vec2::new(size*x as f32, size*y as f32);
                            graphics::draw(ctx, &self.red_square, graphics::DrawParam::new()
                            .dest(dst)
                            .scale(scale),)?;
                        }
                    else if (x + y) % 2 == 0 {
                        graphics::draw(ctx, &self.black_square, graphics::DrawParam::new()
                        .dest(dst)
                        .scale(scale),)?;
                    }
                    else {
                        graphics::draw(ctx, &self.white_square, graphics::DrawParam::new()
                        .dest(dst)
                        .scale(scale),)?;
                    }
                }

                // Draw the chess piece
                let chess_piece = self.game.get_board_piece_clone(BoardPosition::new(x, y));
                if chess_piece.is_some() {
                    let chess_piece = chess_piece.unwrap();

                    if grabbed_piece_pos.is_some() 
                        && grabbed_piece_pos.unwrap().x == x 
                        && grabbed_piece_pos.unwrap().y == y {
                        grabbed_piece = Some(chess_piece);
                        // Draw the piece later ontop of other pieces
                    }
                    else  {
                        let image: Option<&graphics::Image> = self.get_board_piece_image(chess_piece);
                        //let image = Some(&self.black_rook);
                        if image.is_some() {
                            let image = image.unwrap();
                            let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
                            let scale = glam::Vec2::new(scale_factor, scale_factor);
                            let size = image.dimensions().h * scale.x;
                            let dst = glam::Vec2::new(size*x as f32, size*y as f32);
                            graphics::draw(ctx, image, graphics::DrawParam::new()
                                .dest(dst)
                                .scale(scale),)?;
                        }
                    }
                }
            }
        }

        // Draw if player has grabbed a piece
        if grabbed_piece_pos.is_some() && grabbed_piece.is_some() {
            //let image = Some(&self.black_rook);
            let image = self.get_board_piece_image(grabbed_piece.unwrap());
            if image.is_some() {
                let image = image.unwrap();
                let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
                let scale = glam::Vec2::new(scale_factor, scale_factor);
                let size = image.dimensions().h * scale.x;
                let mouse_pos = ggez::input::mouse::position(ctx);
                let dst = glam::Vec2::new(mouse_pos.x - (size*0.5), mouse_pos.y - (size*0.5));
                graphics::draw(ctx, image, graphics::DrawParam::new()
                    .dest(dst)
                    .scale(scale),)?;
            }
        }
        return Ok(());
    }

    pub fn move_chess_piece(&mut self, board_move: BoardMove, ) {
        if self.client1.is_none() {
            let move_result = self.game.move_piece(board_move, true, Some(ChessPieceId::Queen));
            if move_result.is_err() {
                println!("{}", "not a valid move");
            }
        }
        else {
            // Send a request to server to move piece
            let _ = self.client1.as_mut().unwrap().send_move_request(board_move, Some(ChessPieceId::Queen), &mut self.game);
            if self.client2.is_some() {
                // Try to move with the other client as well
                let _ = self.client2.as_mut().unwrap().send_move_request(board_move, Some(ChessPieceId::Queen), &mut self.game);
            }
        }
    }
}

pub fn get_mouse_position(ctx: &mut Context) -> ggez::mint::Point2<f32> {
    ggez::input::mouse::position(ctx)
}


pub fn draw_rectangle(ctx: &mut Context, rect: Rect, color: Color) -> GameResult<()> {
    // First we set the color to draw with, in this case all food will be
    // colored blue.
    //let color = [0.0, 0.0, 1.0, 1.0].into();
    // Then we draw a rectangle with the Fill draw mode, and we convert the
    // Food's position into a `ggez::Rect` using `.into()` which we can do
    // since we implemented `From<GridPosition>` for `Rect` earlier.
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.client1.is_some() {
            self.client1.as_mut().unwrap().update(&mut self.game);
        }
        if self.client2.is_some() {
            self.client2.as_mut().unwrap().update(&mut self.game);
        }
        if self.server.is_some() {
            self.server.as_mut().unwrap().update();
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32
    ) {
        // Store where the mouse was pressed down
        self.mouse_button_press_down = Some(ggez::mint::Point2{x, y});
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32
    ) {
        let mouse_up = ggez::mint::Point2{x, y};

        let mouse_up_board_pos = get_square_from_mouse_pos(mouse_up);
        let mouse_down_board_pos = get_square_from_mouse_pos(self.mouse_button_press_down.unwrap());
        if mouse_up_board_pos.is_err() || mouse_down_board_pos.is_err() {
            return;
        }

        let mouse_up_board_pos = mouse_up_board_pos.unwrap();
        let mouse_down_board_pos = mouse_down_board_pos.unwrap();

        let board_move = BoardMove::new(
            mouse_down_board_pos.x, 
            mouse_down_board_pos.y, 
            mouse_up_board_pos.x, 
            mouse_up_board_pos.y);

        self.move_chess_piece(board_move);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        self.draw_chess_board(ctx)?;

        // See if it is your turn
        let your_turn: bool;
        if self.client1.is_some() && self.client2.is_some() {
            your_turn = true;
        }
        else if self.client1.is_none() && self.client2.is_none() {
            your_turn = true;
        }
        else if self.client1.is_some() && self.client1.as_mut().unwrap().player_color == self.game.turn {
            your_turn = true;
        }
        else {
            your_turn = false;
        }
        if !your_turn {
            let image = &self.wait_for_your_turn;
            let scale_factor = (SCREEN_WIDTH) / (image.dimensions().h*8.0);
            let scale = glam::Vec2::new(scale_factor, scale_factor);
            //let size = image.dimensions().h * scale.x;
            let dst = glam::Vec2::new(SCREEN_WIDTH/2 as f32, SCREEN_WIDTH/2 as f32);
            let offset = glam::Vec2::new(0.5, 0.5);
            graphics::draw(ctx, image, graphics::DrawParam::new()
            .dest(dst)
            .offset(offset)
            .scale(scale),)?;
        }


        graphics::present(ctx)
    }
}