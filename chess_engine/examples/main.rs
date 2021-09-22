pub extern crate chess_engine;
use std::{
    io::{self, BufRead},
};

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
    println!("Algebraic notation, ex Na6 to move knight to a6.");
    println!("Or, print possible moves: ex 'moves a7' to print all moves for a7");
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
                // Treat it as a algebraic move
                let result = game.algebraic_notation_move(user_input[0].clone());
                if result.is_ok() {
                    println!("Move Succesfull!");
                } else {
                    println!("Move Failed!");
                    let error_message = result.err().unwrap();
                    println!("{}", error_message);
                }
                print_game_info(&mut game);
            }
            else if user_input.len() == 2 {
                // Print all possible moves for square
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
            } else {
                println!("Invalid input");
            }
        }
    }
}
