use crate::board::*;

pub fn check_win_conditions(board: &Board) -> (bool, Slot){
    //Check all vertical lines
    for c in 0..3{
        let res = check_verticals(board, c);
        if res.0 {return res;} else {continue;}
    }

    //Check all horizontal lines
    for r in 0..3{
        let res = check_horizontals(board, r);
        if res.0 {return res;} else {continue;}
    }

    let res = check_diagonals(board);
    if res.0 {return res;}

    let res = check_diagonals_rev(board);
    if res.0 {return res;}
    
    (false, Slot::Empty)
}

//Check vertical lines for winners
fn check_verticals(board: &Board, col: usize) -> (bool, Slot){
    
    let mut last_slot = board.slot_by_idx(0, col).unwrap();
    let mut has_won: bool = false;

    for r in 0..2{
        if let Ok(slot) = board.slot_by_idx(r, col){
            if slot != Slot::Empty && slot == board.slot_by_idx(r+1, col)
            .unwrap_or(Slot::Empty){
                last_slot = slot;
            }else{
                has_won = false;
                return (false, Slot::Empty);
            }
        }
    }
    
    (true, last_slot)
}

//Check horizontal lines for winners
fn check_horizontals(board: &Board, row: usize) -> (bool, Slot){
    
    let mut last_slot = board.slot_by_idx(row, 0).unwrap();
    let mut has_won: bool = false;

    for c in 0..2{
        if let Ok(slot) = board.slot_by_idx(row, c){
            if slot != Slot::Empty && slot == board.slot_by_idx(row, c+1)
            .unwrap_or(Slot::Empty){
                last_slot = slot;
            }else{
                has_won = false;
                return (false, Slot::Empty);
            }
        }
    }
    
    (true, last_slot)
}

//Check the diagonal line 1 for winners
pub fn check_diagonals(board: &Board) -> (bool, Slot){
    let mut last_slot = board.slot_by_idx(0, 0).unwrap();
    let mut has_won: bool = false;

    for c in 0..2{
        if let Ok(slot) = board.slot_by_idx(c, c){
            if slot != Slot::Empty && slot == board.slot_by_idx(c+1, c+1)
            .unwrap_or(Slot::Empty){
                last_slot = slot;
            }else{
                has_won = false;
                return (false, Slot::Empty);
            }
        }
    }
    
    (true, last_slot)
}

//Check the diagonal line 2 for winners
pub fn check_diagonals_rev(board: &Board) -> (bool, Slot){
    let mut last_slot = board.slot_by_idx(0, 2).unwrap();
    let mut has_won: bool = false;

    for c in (1..3).rev(){
        if let Ok(slot) = board.slot_by_idx(2-c, c){
            if slot != Slot::Empty && slot == board.slot_by_idx(2-c+1, c-1)
            .unwrap_or(Slot::Empty){
                last_slot = slot;
            }else{
                has_won = false;
                return (false, Slot::Empty);
            }
        }
    }
    
    (true, last_slot)
}