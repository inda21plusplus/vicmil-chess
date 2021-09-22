pub extern crate chess_engine;
use std::{
    io::{self, BufRead},
    num::ParseIntError,
};

use chess_engine::chess_game::{BoardMove, ChessPieceId};

fn print_game_info(game: &mut chess_engine::chess_game::Game) {
    game.print_board();
    use chess_engine::chess_game::*;
    if game.turn as u32 == ChessPieceColor::Black as u32 {
        println!("Black's turn");
    } else {
        println!("White's turn");
    }
    let check = game.is_check();
    if check.is_some() {
        println!("Check!");
        print!("{} {} {} {}", check.unwrap().from_x, check.unwrap().from_y, check.unwrap().to_x, check.unwrap().to_y);
    }
    println!();
    println!("from_x from_y to_x to_y, example '0 1 0 2'");
    println!("Or: 'from_x from_y type', example '0 0 knight'");
}
fn main() {
    let mut game = chess_engine::chess_game::Game::new();
    game.set_up_board();
    loop {
        let stdin = io::stdin();
        print_game_info(&mut game);
        for line in stdin.lock().lines().map(|l| l.unwrap()) {
            let user_input: Vec<String> =
                line.split_whitespace().map(|num| num.to_string()).collect();
            if user_input.len() == 1 {
                let result = game.algebraic_notation_move(user_input[0].clone());
                if result.is_ok() {
                    println!("Move Succesfull!");
                } else {
                    let error_message = result.err().unwrap();
                    println!("{}", error_message);
                }
                print_game_info(&mut game);
            }
            else if user_input.len() == 2 {
                if user_input[0] == "moves" {
                    let position = user_input[1].clone();
                    if position.len() != 2 {
                        println!("Invalid input");
                    }
                    let char_vec: Vec<char> = position.chars().collect();
                    let result_letter = game.get_coordinate_from_letter(char_vec[0]);
                    let result_number = game.get_coordinte_from_number(char_vec[1]);
                    if result_letter.is_ok() && result_number.is_ok() {
                        game.print_board_with_possible_moves(true, result_letter.unwrap(), result_number.unwrap());
                    }
                    else {
                        println!("Invalid input");
                    }
                }
            }
            else if user_input.len() == 4 {
                let from_x: Result<u32, ParseIntError> = user_input[0].parse();
                let from_y: Result<u32, ParseIntError> = user_input[1].parse();
                let to_x: Result<u32, ParseIntError> = user_input[2].parse();
                let to_y: Result<u32, ParseIntError> = user_input[3].parse();
                if from_x.is_ok() && from_y.is_ok() && to_x.is_ok() && to_y.is_ok() {
                    let board_move = BoardMove::new(
                        from_x.unwrap() as u8,
                        from_y.unwrap() as u8,
                        to_x.unwrap() as u8,
                        to_y.unwrap() as u8,
                    );
                    println!();
                    println!();
                    let result = game.move_piece(board_move, true);
                    if result.is_ok() {
                        println!("Move Succesfull!");
                    } else {
                        println!("Invalid move!");
                        let error_message = result.err().unwrap().clone();
                        println!("{}", error_message);
                    }
                    print_game_info(&mut game);
                } else {
                    println!("Invalid input");
                }
            } else if user_input.len() == 3 {
                if user_input[0] == "moves" {
                    // Print possible moves for piece
                    let from_x: Result<u32, ParseIntError> = user_input[1].parse();
                    let from_y: Result<u32, ParseIntError> = user_input[2].parse();
                    if from_x.is_ok() && from_y.is_ok() {
                        game.print_board_with_possible_moves(true, from_x.unwrap() as u8, from_y.unwrap() as u8);
                    }
                }
                else {
                    let from_x: Result<u32, ParseIntError> = user_input[0].parse();
                    let from_y: Result<u32, ParseIntError> = user_input[1].parse();
                    let to_id = ChessPieceId::from_str(&user_input[2].as_str());
                    if from_x.is_ok() && from_y.is_ok() && to_id.is_ok() {
                        println!();
                        println!();
                        let result =
                            game.promote(from_x.unwrap() as u8, from_y.unwrap() as u8, to_id.unwrap());
                        if result.is_ok() {
                            println!("Move Succesfull!");
                        } else {
                            println!("Invalid move!");
                            let error_message = result.err().unwrap().clone();
                            println!("{}", error_message);
                        }
                    } else {
                        println!("Invalid input");
                    }
                }
                print_game_info(&mut game);
            } else {
                println!("Invalid input");
            }
        }
    }
}
