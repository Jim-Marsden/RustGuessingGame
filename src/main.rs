mod game;
mod my_print;

use rand::Rng;

fn main(){

    // let target_num = rand::thread_rng().gen_range(1..10);
    match game::game(rand::thread_rng().gen_range(1..10), 4, my_print::get_input){
        true => println!("You are winner!"),
        false => println!("You are loser :c")
    };
}

