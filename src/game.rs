use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq)]
pub enum GuessDirection {
    High,
    Low,
    Correct,
}

impl fmt::Display for GuessDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GuessDirection::High => write!(f, "high"),
            GuessDirection::Low => write!(f, "low"),
            GuessDirection::Correct => write!(f, "correct"),
        }
    }
}


pub fn turn(target: i32, guess: i32) -> GuessDirection {
    if guess < target {
        return GuessDirection::Low;
    } else if guess > target {
        return GuessDirection::High;
    } else {
        return GuessDirection::Correct;
    }
}

fn turn_start_msg(turn: i32, range_max: i32) {
    println!("Guess a number. (this is your {} guess out of {})",
                   match turn {
                       1 => "1st".to_string(),
                       2 => "2nd".to_string(),
                       3 => "3rd".to_string(),
                       _ => format!("{}th", turn)
                   }, range_max);
}

pub fn game(target_number:i32, range_max:i32, get_int:fn()->i32) -> bool {
    let min_turn:i32 = 0;
   for n in min_turn..range_max{
        turn_start_msg(n + 1, range_max);

        let result = turn(target_number, get_int());
        if result == GuessDirection::Correct {
            return true;
        }
        println!("Your guess was {}.", result);
    }

    return false;
}