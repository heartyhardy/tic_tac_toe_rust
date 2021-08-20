
pub const MAX_TURNS:u8 = 6;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Turns{
    PlayerOneTurn = 1,
    PlayerTwoTurn = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerTurn{
    player_one_turns: u8,
    player_two_turns: u8,
    next_turn: [Turns; 2],
}

//Associate functions
impl PlayerTurn{

    pub fn new() -> PlayerTurn{
        PlayerTurn{
            player_one_turns: 0,
            player_two_turns: 0,
            next_turn: [Turns::PlayerOneTurn, Turns::PlayerTwoTurn]
        }
    }
}

//Methods
impl PlayerTurn{

    //Get current turn owner
    pub fn current_turn(&self) -> Turns{
        self.next_turn[0]
    }

    //Proceed to next turn by swapping elements of the 'next_turn' array
    pub fn next_turn(&mut self) -> Turns{
        self.next_turn.swap(0,1);
        self.next_turn[0]
    }

    //Increment currently active Player's turns taken count
    pub fn increment_turns(&mut self){
        if self.next_turn[0] == Turns::PlayerOneTurn{
            self.player_one_turns += 1;
        }else if self.next_turn[0] == Turns::PlayerTwoTurn {
            self.player_two_turns += 1;
        }
    }

    //Get specified Player's turn count
    pub fn get_turns(&self, turn_owner: Turns) -> u8{
        if turn_owner == Turns::PlayerTwoTurn{
            self.player_one_turns
        }else if turn_owner == Turns::PlayerTwoTurn{
            self.player_two_turns
        }else{
            0
        }
    }

    //Get total turns
    pub fn get_total_turns(&self) -> u8{
        self.player_one_turns + self.player_two_turns
    } 
}



