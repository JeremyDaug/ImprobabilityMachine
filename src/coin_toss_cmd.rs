use std::{io::{self, stdin, stdout, Write}, thread, time::{Duration, Instant}};

use crossterm::{cursor::{self, MoveToPreviousLine, SavePosition}, event::{poll, read, KeyEvent}, execute, style::Print, terminal, ExecutableCommand, QueueableCommand};
use macroquad::miniquad::native::linux_x11::libx11::KeyCode;

use crate::{coin, coin_toss::{CoinToss, CoinTossState}, common_state::CommonState};

pub fn select_screen(common_state: &mut CommonState, coin_toss: &mut CoinToss, start: Instant) -> Option<CoinTossState> {
    match coin_toss.state {
        CoinTossState::Hold => {
            holding_screen(common_state, coin_toss, start)
        },
        CoinTossState::StartBet => {
            start_bet(common_state, coin_toss, start)
        },
        CoinTossState::InBet => todo!(),
        CoinTossState::ClosingBet => todo!(),
    }
}

/// # Start Bet
/// 
/// Starts the bet, flips coin a few times, then lands on it's head.
pub fn start_bet(common_state: &CommonState, coin_toss: &CoinToss, start: Instant) -> Option<CoinTossState> {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let flip_start = Instant::now();
    let mut msg = String::new();
    loop {
        let time = Instant::now() - start;
        let side = (time.as_secs_f32() * 10.0) as i32 % 2;
        let from_start = Instant::now() - flip_start;
        // 
        stdout().write_all("\t\t!!!Coin Toss!!!\nCommands: F -> Flip | Q -> Exit | Enter number to change Bet\nBet Min: $1 | Bet Max: $100\n".as_bytes()).unwrap();
        stdout().write_all(format!("Money: ${}\tEntropy: {} b\n", common_state.money, common_state.entropy).as_bytes()).unwrap();
        stdout().execute(Print(format!("Current Bet: {}\n", coin_toss.base.current_bet))).unwrap();
        stdout().execute(Print(msg.as_str())).unwrap();
        if side == 1 {
            stdout().execute(Print("\t\tH\n")).unwrap();
        } else {
            stdout().execute(Print("\t\t\tT\n")).unwrap();
        }
        // after three seconds, finish the coin toss and start the bet proper.
        if Duration::from_secs(3) < from_start {
            //common_state.
            return Some(CoinTossState::InBet);
        }
    }
}

/// # Holding screen
/// 
/// Should show a holding screen with the user's current Money, Entropy, and 
/// the current outcome of the coin toss game.
/// 
/// It should also have a list of commands for the holding state.
pub fn holding_screen(common_state: &mut CommonState, coin_toss: &mut CoinToss, start: Instant) -> Option<CoinTossState> {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut msg = String::new();
    loop {
        let time = ((Instant::now() - start).as_secs_f32() * 2.0) as i32 % 2;
        // Set up bet and promts for it.
        stdout().execute(Print("\t\t!!!Coin Toss!!!\nCommands: F -> Flip | Q -> Exit | Enter number to change Bet\nBet Min: $1 | Bet Max: $100\n")).unwrap();
        stdout().execute(Print(format!("Money: ${}\tEntropy: {} b\n", common_state.money, common_state.entropy))).unwrap();
        stdout().execute(Print(format!("Current Bet: {}\n", coin_toss.base.current_bet))).unwrap();
        stdout().execute(Print(msg.as_str())).unwrap();
        stdout().execute(Print("\n\t\t H or T? \n")).unwrap();
        // swap the H / T every half second.
        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        println!("{}, {}", buff, buff.len());
        buff = buff.trim_end().to_string();
        //buff.parse::<f64>().unwrap();
        if let Ok(bet) = buff.parse::<f64>() {
            if bet < coin_toss.base.bet_min || bet > coin_toss.base.bet_max {
                msg = String::from("Bet must be within bounds!");
            } else {
                coin_toss.base.current_bet = bet.floor();
            }
        } else if buff.to_lowercase() == "f" {
            stdout().execute(Print("Flipping!")).unwrap();
            return Some(CoinTossState::StartBet);
        } else if buff.to_lowercase() == "q" {
            stdout().execute(Print("Quitting!")).unwrap();
            return None;
        } else {
            msg = String::from("Invalid Command.");
        }

        thread::sleep(Duration::from_secs(1));
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    }
}