use crate::turns::{self, Turns};
use crate::utils::{clear};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Slot{
    Empty = 0,
    PlayerOne = 1,
    PlayerTwo = 2
}

pub struct Board{
    pub slots:[[Slot;3];3]
}

//Associate functions
impl Board{
    pub fn new() -> Board{
        Board{
            slots: [[Slot::Empty;3]; 3]
        }
    }

    //Turn to Slot
    pub fn turn_to_slot(turn:Turns) -> Slot{
        match turn{
            Turns::PlayerOneTurn => Slot::PlayerOne,
            Turns::PlayerTwoTurn => Slot::PlayerTwo,
        }
    }
}

//Methods
impl Board{

    //Reset gameboard
    pub fn reset(&mut self){
        for row in self.slots.iter_mut(){
            for col in row{
                *col = Slot::Empty
            }
        }
    }

    //Mark a slot with PlayerName
    pub fn mark_slot(&mut self, row:usize, col:usize, player_slot: Slot) -> bool{
        if row > 2 || col > 2{
            return false;
        }
        self.slots[row][col] = player_slot;
        true
    }

    //Get slot
    pub fn slot_by_idx(&self, row:usize, col:usize) -> Result<Slot, ()>{
        if row > 2 || col > 2{
            return Err(());
        }

        Ok(self.slots[row][col])
    }

    //Is this slot already occupied?
    pub fn is_occupied(&self, row:usize, col:usize) -> Result<bool,()>{
        
        if row > 2 || col > 2{
            return Err(());
        }

        if self.slots[row][col] != Slot::Empty {Ok(true)} else {Ok(false)}
    }

    //Print the board to the console
    pub fn display_board(&self){
        clear();
        for r in 0..3_usize{
            for c in  0..3_usize{
                print!(" {} ", self.slots[r][c] as usize);
            }
            println!();
        }
    }

}