pub(crate) mod money;
pub mod game;
pub mod common_state;
pub mod coin_game;
pub mod gfx;
pub mod machine;
pub mod main_menu;
pub mod bang;

use std::{env, io::{stdin, stdout}, time::{Duration, Instant}};

use crossterm::{ExecutableCommand, style::Print};
use ::rand as stdrng;

use bevy::prelude::*;

use crate::{coin_game::{coin_toss::CoinToss, coin_toss_cmd::select_screen}, common_state::{ButtonAction, CommonState}, gfx::coin::Coin, machine::machine::Machine, main_menu::main_menu};

fn main() {
    App::new().run();
}

fn _old_main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let start_time = Instant::now();
    let mut common_state = CommonState { 
        money: 20.0*12.0, 
        entropy: 100.0, 
        active_game: 0, 
        current_bet: 10.0, 
        button_clicked: ButtonAction::None,
        machine: Machine::new(0.0) ,
        player_name: String::new(),
        last_prior_save: Instant::now(),
        game_length: Duration::ZERO
    };
    let mut coin_toss = CoinToss::new();
    let mut rng = stdrng::rng();

    if mode == "cmd" {
        println!("\n\n\n\n\n\n\n\n");
        println!("-------------- Command Line Interface Selected. Starting up -----------");
        println!("\n\n\n\n\n");

        main_menu(&mut common_state);
    } else if mode == "ui" {
        let mut change = 0.0;
    } else if _is_help_cmd(mode) {
        println!("The Improbability machine has a 2 modes it can run in.\n");
        println!("cmd: Command Line mode. Used for more direct debugging. Very basic.");
        println!("ui: The Game UI that will be used. Currently only barely functional, don't expect much.");
    } else if mode.to_lowercase() == "tools" {
        loop {
            stdout().execute(Print("Tools menu:\n\n")).unwrap();
            stdout().execute(Print("(1) : Bang Probability Calculations\n")).unwrap();
            stdout().execute(Print("(q) : Quit\n\n")).unwrap();
            let mut buff = String::new();
            stdin().read_line(&mut buff).unwrap();
            buff = buff.trim().to_string();
            if buff == "1" {
                bang::bang_probs();
            } else if buff.to_lowercase() == "q" {
                break;
            }
        }
    } else {
        println!("Mode command not given. Try -- help for modes")
    }
}

fn _is_help_cmd(arg: &String) -> bool {
    arg == "help" ||
    arg == "Help" ||
    arg == "h" ||
    arg == "H"
}

struct Point {
    pub x: f32,
    pub y: f32
}