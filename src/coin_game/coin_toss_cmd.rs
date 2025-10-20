use std::{io::{stdin, stdout}, thread::sleep, time::{Duration, Instant}};

use crossterm::{event::{poll, read, Event, KeyCode}, style::Print, terminal, ExecutableCommand};
use rand::Rng;

use crate::{coin_game::coin_toss::{CoinToss, CoinTossState}, common_state::CommonState};

pub fn select_screen<R: Rng>(common_state: &mut CommonState, 
coin_toss: &mut CoinToss, start: Instant, rng: &mut R) -> Option<CoinTossState> {
    match coin_toss.state {
        CoinTossState::Hold => {
            holding_screen(common_state, coin_toss, start)
        },
        CoinTossState::StartBet => {
            start_bet(common_state, coin_toss, start)
        },
        CoinTossState::InBet => {
            in_bet(common_state, coin_toss, start, rng)
        },
    }
}

pub fn in_bet<R: Rng>(common_state: &mut CommonState, coin_toss: &mut CoinToss, 
_start: Instant, rng: &mut R) -> Option<CoinTossState> {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    // subtract money for bet
    common_state.money -= coin_toss.base.current_bet;
    // Commit flip
    coin_toss.result = coin_toss.bet(rng);
    // save entropy
    let entropy_gained = coin_toss.entropy_gained();
    common_state.add_entropy(entropy_gained);
    // start timer
    coin_toss.base.bet_start = Some(Instant::now());
    loop {
        // print screen and timer.
        stdout().execute(
            Print("\t\t!!!Coin Toss!!!\nLand on heads to win!
            Commands: F -> Flip again (0.5 Entropy Cost) | W -> Select Heads (1 Entropy Cost) |
            L -> Select Tails (1 Entropy Cost) | Q -> End Bet
            Bet Min: $1 | Bet Max: $100\n")).unwrap();
        stdout().execute(
            Print(format!("Money: ${}\tEntropy: {}b\tSuspicion: {}\n", common_state.money, common_state.entropy, coin_toss.base.suspicion))
        ).unwrap();
        stdout().execute(Print(format!("Current Bet: {}\t Entropy Gained: {}\n", coin_toss.base.current_bet, entropy_gained))).unwrap();
        stdout().execute(Print(format!("Time Remaining: {} s\n", coin_toss.bet_time_remaining()))).unwrap();
        if coin_toss.result {
            stdout().execute(Print("\t\tH\t! You're Winner !\n")).unwrap();
        } else {
            stdout().execute(Print("\t\t\tT\t! FAILURE !\n")).unwrap();
        }
        // Get key presses while looping.
        if poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                if event.code == KeyCode::Char('f') {
                    // Flip coin again, ignore whether the player has won or lost.
                    common_state.entropy -= 0.5;
                    coin_toss.result = coin_toss.flip(rng);
                } else if event.code == KeyCode::Char('w') {
                    // force coin to heads
                    common_state.entropy -= 1.0;
                    coin_toss.result = true;
                } else if event.code == KeyCode::Char('l') {
                    // force coin to tails
                    common_state.entropy -= 1.0;
                    coin_toss.result = false;
                } else if event.code == KeyCode::Char('q') {
                    // exiting bet early.
                    break;
                }
            }
        }
        // lostly check that the bet is over. If it is, close out and move on.
        if coin_toss.bet_time_remaining() == 0.0 {
            break;
        }
        sleep(Duration::from_millis(50));
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    }
    // finalize financial gains if successful.
    if coin_toss.result {
        common_state.money += coin_toss.base.base_payout * coin_toss.base.current_bet;
    }
    coin_toss.state = CoinTossState::Hold;
    return Some(CoinTossState::Hold);
}

/// # Start Bet
/// 
/// Starts the bet, flips coin a few times, then lands on it's head.
pub fn start_bet(common_state: &CommonState, coin_toss: &mut CoinToss, start: Instant) -> Option<CoinTossState> {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let flip_start = Instant::now();
    let msg = String::new();
    //thread::sleep(Duration::from_secs(1));
    loop {
        //sleep(Duration::from_millis(250));
        let time = Instant::now() - start;
        let side = (time.as_secs_f32() * 10.0) as i32 % 2;
        let from_start = Instant::now() - flip_start;
        // 
        stdout().execute(Print("\t\t!!!Coin Toss!!!\nLand on heads to win!\nCommands: F -> Flip | Q -> Exit | Enter number to change Bet\nBet Min: $1 | Bet Max: $100\n")).unwrap();
        stdout().execute(Print(format!("Money: ${}\tEntropy: {} b\n", common_state.money, common_state.entropy))).unwrap();
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
            coin_toss.state = CoinTossState::InBet;
            return Some(CoinTossState::InBet);
        }
        sleep(Duration::from_millis(100));
        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
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
        //println!("{}, {}", buff, buff.len());
        buff = buff.trim_end().to_string();
        //buff.parse::<f64>().unwrap();
        if let Ok(bet) = buff.parse::<f64>() {
            if bet < coin_toss.base.bet_min || bet > coin_toss.base.bet_max {
                msg = String::from("Bet must be within bounds!");
            } if bet > common_state.money {
                msg = String::from("Not enough money!!");
            } else {
                coin_toss.base.current_bet = bet.floor();
            }
        } else if buff.to_lowercase() == "f" {
            stdout().execute(Print("Flipping!")).unwrap();
            coin_toss.state = CoinTossState::StartBet;
            return Some(CoinTossState::StartBet);
        } else if buff.to_lowercase() == "q" {
            stdout().execute(Print("Quitting!")).unwrap();
            return None;
        } else {
            msg = String::from("Invalid Command.");
        }

        if common_state.money < 1.0 {
            stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
            stdout().execute(Print("!!! Ran out of Money! Game Over !!!")).unwrap();
            stdin().read_line(&mut buff).unwrap();
            return None;
        }

        stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    }
}