use std::{fs::{self, File}, io::{stdin, stdout, Read, Write}, path::Path, time::{Duration, Instant}};

use crossterm::{style::Print, terminal, ExecutableCommand};
use ::rand as stdrng;

use crate::{coin_game::{coin_toss::{self, CoinToss, CoinTossState}, coin_toss_cmd}, common_state::{self, ButtonAction, CommonState}, machine::machine::Machine};

static SAVE_PATH: &str = "./save/save.txt";

pub fn main_menu(common_state: &mut CommonState) {
    let mut msg = String::new();
    let mut common_state = CommonState {
        money: 100.0,
        entropy: 100.0,
        active_game: 0,
        current_bet: 1.0,
        button_clicked: ButtonAction::None,
        machine: Machine::new(0.0),
        player_name: String::from(""),
        last_prior_save: Instant::now(),
        game_length: Duration::ZERO
    };
    loop {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().execute(Print("!!!!!!!!!! Improbability Machine !!!!!!!!!!\n\n")).unwrap();
        stdout().execute(Print("Commands:\n")).unwrap();
        stdout().execute(Print("(N)ew Game (Overwrite's old save)\n")).unwrap();
        stdout().execute(Print("(L)oad Save\n")).unwrap();
        stdout().execute(Print("(Q)uit\n\n")).unwrap();
        stdout().execute(Print(format!("{}\n\n", msg))).unwrap();
        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        buff = buff.trim().to_string();
        if buff.to_lowercase() == "q" {
            break;
        } else if buff.to_lowercase() == "n" {
            // create new data and save over old.
            stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
            stdout().execute(Print("Please input your name:\n")).unwrap();
            let mut buff = String::new();
            stdin().read_line(&mut buff).unwrap();
            common_state = new_save_file(buff);
            common_state.last_prior_save = Instant::now();
            game_menu(&mut common_state);
        } else if buff.to_lowercase() == "l" {

        } else {
            msg = String::from("Command not recognized.");
        }
    }
}

pub fn game_menu(common_state: &mut CommonState) {
    let mut rng = stdrng::rng();
    let mut msg = String::new();
    loop {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().execute(Print("!!!!!!!!!! Improbability Machine !!!!!!!!!!\n\n")).unwrap();
        stdout().execute(Print(format!("Money: ${}\tEntropy: {} b\n", common_state.money, common_state.entropy))).unwrap();
        stdout().execute(Print("Game Commands:\n")).unwrap();
        stdout().execute(Print("(1) Coin Toss\n")).unwrap();
        stdout().execute(Print("(S)ave Game\n")).unwrap();
        stdout().execute(Print("(Q) Return to Main Menu\n\n")).unwrap();
        stdout().execute(Print(format!("{}\n\n", msg))).unwrap();
        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        buff = buff.trim().to_string();
        if buff == "1" {
            let mut coin_toss = CoinToss::new();
            common_state.current_bet = coin_toss.base.bet_min;
            loop {
                if let Some(res) = coin_toss_cmd::select_screen(common_state, &mut coin_toss, 
                common_state.last_prior_save, &mut rng) {
                    coin_toss.state = res;
                } else {
                    break;
                }
            }
        } else if buff == "S" {
            save_common_state(common_state);
            msg = String::from("!!!!! Saved !!!!!!")
        } else if buff == "Q" {
            return;
        }
    }
}

pub fn save_common_state(common_state: &mut CommonState) {
    let path = Path::new(SAVE_PATH);
    let mut file = File::create(path).unwrap();
    file.write(common_state.save_str().as_bytes()).unwrap();
}

pub fn new_save_file(player_name: String) -> CommonState {
    let path = Path::new(SAVE_PATH);
    let mut new_data = CommonState::new(player_name);
    let mut file = File::create(path).unwrap();
    file.write(new_data.save_str().as_bytes()).unwrap();
    new_data
}

pub fn load_save() -> CommonState {
    let path = Path::new(SAVE_PATH);
    let file = fs::read_to_string(path)
        .expect("Could not read file.");
    let mut new_state = CommonState::empty();
    new_state.load_state(file);
    new_state
}