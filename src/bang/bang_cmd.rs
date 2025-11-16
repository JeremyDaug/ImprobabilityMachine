use std::{io::{stdin, stdout}, thread::sleep, time::{Duration, Instant}};

use crossterm::{ExecutableCommand, event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read}, style::Print, terminal};
use rand::Rng;

use crate::{bang::bang::{Bang, BangState}, common_state::CommonState};



pub fn select_screen<R: Rng>(common_state: &mut CommonState,
bang: &mut Bang, start: Instant, rng: &mut R) -> Option<BangState> {
    match bang.state {
        super::bang::BangState::HoldingScreen => {
            holding_screen(common_state, bang, start)
        },
        super::bang::BangState::StartBet => {
            start_bet(common_state, bang, start, rng)
        },
        super::bang::BangState::InBet => {
            in_bet(common_state, bang, start, rng)
        },
    }
}

pub fn in_bet<R: Rng>(common_state: &mut CommonState, bang: &mut Bang, 
start: Instant, rng: &mut R) -> Option<BangState> {
    bang.base.bet_start = Some(Instant::now());
    bang.bet(rng);
    common_state.money -= bang.base.current_bet;
    let entropy_gained = bang.entropy_gained();
    common_state.add_entropy(entropy_gained);
    let (player_score, opponent_score) = bang.scoring();
    let mut msg = String::new();
    let mut player_input = String::new();
    let mut execute_input = false;
    let mut close_out = false;
    loop {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().execute(Print("\t!!!BANG!!!\n")).unwrap();
        stdout().execute(Print("\tCommands: R -> Roroll all | Q -> Exit | X -> Show Rules\n")).unwrap();
        stdout().execute(Print("\tR# -> Reroll Specific Dice (0-4 player, 5-9 opponent) | F# -> Fix Die (0-4 player, 5-9 opponent) | E# -> Exclude outcome (and reroll) (0-4 player, 5-9 opponent)\n")).unwrap();
        stdout().execute(Print(format!("\tBet Min: ${}\tBet Max: ${}\n", 
            bang.base.bet_min, bang.base.bet_max))).unwrap();
        stdout().execute(Print(format!("\tMoney: ${}\tEntropy: {} b\n", 
            common_state.money, common_state.entropy))).unwrap();
        stdout().execute(Print(format!("\tCurrent Bet: ${}\n", bang.base.current_bet))).unwrap();
        stdout().execute(Print(format!("\t\tTime Remaining: {} sec", bang.bet_time_remaining()))).unwrap();
        stdout().execute(Print(format!("\nEntropy Gained: {}\n\n",entropy_gained))).unwrap();
        stdout().execute(Print(format!("\n{}\n\n",msg))).unwrap();
        let roll_result = bang.get_rolls();
        stdout().execute(Print(roll_result)).unwrap();
        stdout().execute(Print(format!("Player Score: {}\t Opponent Score: {}\n", player_score, opponent_score))).unwrap();
        stdout().execute(Print(format!("Player Score: {}\n", bang.possibilities))).unwrap();
        if player_score > opponent_score {
            stdout().execute(Print("\t\t! Your Winner !\n")).unwrap();
        } else {
            stdout().execute(Print("\t\t! FAILURE !\n")).unwrap();
        }
        stdout().execute(Print(format!(">{}\n", player_input))).unwrap();
        
        if poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(KeyEvent { code, kind, .. }) = read().unwrap() {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Backspace => {
                            // if backspace, jump to back.
                            player_input.pop();
                        },
                        KeyCode::Char(c) => {
                            player_input.push(c);
                        },
                        KeyCode::Enter => {
                            execute_input = true;
                        },
                        _ => {}
                    }
                }
            }
        }
        if execute_input { // if enter hit, run player_input
            execute_input = false;
            msg = String::new();
            // Shift to lower case.
            player_input = player_input.to_lowercase();
            if player_input.len() == 0 {
            } else if player_input.chars().nth(0).unwrap() == 'r' {
                // Do Reroll and remove entropy.
                bang.roll_dice(rng);
            } else if player_input.chars().nth(0).unwrap() == 'q' {
                // Q Hard quit for now.
                close_out = true;
            } else if player_input.chars().nth(0).unwrap() == 'f' {

            } else if player_input.chars().nth(0).unwrap() == 'e' {

            } else {
                player_input = String::new();
                msg = String::from("Invalid Command");
            }
        }
        if bang.bet_time_remaining() == 0.0 || close_out {
            // wrap up bet
            bang.state = BangState::HoldingScreen;
            return Some(BangState::HoldingScreen);
        }
    }
}

pub fn start_bet<R: Rng>(common_state: &CommonState, bang: &mut Bang, _start: Instant, rng: &mut R) 
-> Option<BangState> {
    let roll_start = Instant::now();
    let msg = String::new();
    loop {
        //let time = Instant::now() - start;
        let from_start = Instant::now() - roll_start;
        bang.roll_dice(rng);
        // text
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout().execute(Print("\t\t!!!BANG!!!\n\n")).unwrap();
        //stdout().execute(Print("\tCommands: R -> Roll | Q -> Exit | X -> Show Rules\n")).unwrap();
        stdout().execute(Print(format!("\tBet Min: ${}\tBet Max: ${}\n", 
            bang.base.bet_min, bang.base.bet_max))).unwrap();
        stdout().execute(Print(format!("\tMoney: ${}\tEntropy: {} b\n", 
            common_state.money, common_state.entropy))).unwrap();
        stdout().execute(Print(format!("\tCurrent Bet: ${}\n", bang.base.current_bet))).unwrap();
        stdout().execute(Print(format!("\n{}\n\n",msg))).unwrap();
        let roll_result = bang.get_rolls();
        stdout().execute(Print(roll_result)).unwrap();

        if Duration::from_secs(3) < from_start {
            bang.state = BangState::InBet;
            return Some(BangState::InBet);
        }
        sleep(Duration::from_millis(500));
    }
}

pub fn holding_screen(common_state: &mut CommonState, bang: &mut Bang, start: Instant) 
-> Option<BangState> {
    let mut msg = String::new();
    loop {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        // set up show
        stdout().execute(Print("\t\t!!!BANG!!!\n")).unwrap();
        stdout().execute(Print("\tCommands: R -> Roll | Q -> Exit | Enter Number to change bet | X -> Show Rules\n")).unwrap();
        stdout().execute(Print(format!("\tBet Min: ${}\tBet Max: ${}\n", 
            bang.base.bet_min, bang.base.bet_max))).unwrap();
        stdout().execute(Print(format!("\tMoney: ${}\tEntropy: {} b\n", 
            common_state.money, common_state.entropy))).unwrap();
        stdout().execute(Print(format!("\tCurrent Bet: ${}\n", bang.base.current_bet))).unwrap();
        stdout().execute(Print(format!("\n{}\n\n",msg))).unwrap();
        stdout().execute(Print("Opponent's Roll: 1,2,3,4,5\n\n")).unwrap();
        stdout().execute(Print("Your Roll: 1,2,3,4,5\n\n")).unwrap();
        // read inputs
        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        buff = buff.trim_end().to_string();
        // read commands
        if let Ok(bet) = buff.parse::<f64>() {
            // Change Bet
            if bet < bang.base.bet_min || bet > bang.base.bet_max {
                msg = String::from("Bet must be within Bounds!")
            } else if bet > common_state.money {
                msg = String::from("Not enough Money!!");
            } else {
                msg = String::from("");
                bang.base.current_bet = bet.floor();
            }
        } else if buff.to_lowercase() == "q" {
            stdout().execute(Print("Quitting!")).unwrap();
            return None;
        } else if buff.to_lowercase() == "x" {
            msg = String::from("You and your opponent roll 5 dice each.\nThe number of pips is the base value, and it gets multiplied by pairs and straights.\nmatches: 2 -> 2x, 3 -> 3x, 4 -> 5x, 5 -> 10x\nStraights: 3 -> 3x, 4 -> 5x, 5 -> 8x\n");
        } else if buff.to_lowercase() == "r" {
            stdout().execute(Print("Rolling!")).unwrap();
            bang.state = BangState::StartBet;
            return Some(BangState::StartBet);
        }

        if common_state.money < 10.0 {
            stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
            stdout().execute(Print("!!! Not enough Money !!!")).unwrap();
            stdin().read_line(&mut buff).unwrap();
            return None;
        }
    }
}