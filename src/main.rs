/*

1. Draw board
2. Player 1 turn
3. Player 2 turn
4. If Turn count < MAX_TURNS(6) check Win Conditions
5. If Turn count == MAX_TURNS(6) check Win Conditions
REPEAT from 2

*/

#![allow(warnings)]

mod board;
use crate::board::*;

mod turns;
use crate::turns::*;

mod input;
use crate::input::*;

mod rules;
use crate::rules::*;

mod utils;

fn game_loop() {
    //Init
    let mut game_board = Board::new();
    let mut turns = PlayerTurn::new();

    game_board.display_board();

    loop {
        //check max turns
        if turns.get_total_turns() < MAX_TURNS {
            // Current turns are 4 or less so no need to check win conditions

            // Player one go
            println!("{:?} Enter Row:", turns.current_turn());
            let mut row = get_input(&game_board);
            println!("{:?} Enter Column:", turns.current_turn());            
            let mut col = get_input(&game_board);

            if row != None && col != None {

                let row = row.unwrap() as usize;
                let col = col.unwrap() as usize;

                if let Ok(res) = game_board.is_occupied(row, col) {
                    if !res {
                        //mark player move
                        let slot = Board::turn_to_slot(turns.current_turn());
                        game_board.mark_slot(row, col, slot);
                        //display board
                        game_board.display_board();
                        //increment current player's turn count
                        turns.increment_turns();
                        //swap players
                        turns.next_turn();

                        if turns.get_total_turns() == 5{
                            let has_won = check_win_conditions(&game_board);
                            if has_won.0 {
                                println!("{:?} Won the game!!", has_won.1);
                                break;
                            }
                        }

                    } else {
                        continue;
                    }
                }
            }
        } else {
            //Turn count is 6
            //Need to check win conditions
            let has_won = check_win_conditions(&game_board);
            if has_won.0{
                println!("{:?} Won the game!!", has_won.1);
            }else{
                println!("It's a Draw!");
            }
            break;
        }
    }
}

fn main() {
    game_loop();
}
