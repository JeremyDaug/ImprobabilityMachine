use std::{collections::BTreeMap, fs, ops::IndexMut, path::Path, time::{Duration, Instant}};

use rand::{Rng, distr::Distribution};

use crate::{bang::BANG_CHANCES, game::GameCommonData};


/// # Bang
/// 
/// Bang is effectively a Yatzee Knockoff. It follows similar rules, though instead of
/// it being a multiround game, it's a one and done game.
/// 
/// Points are as follows
/// 
/// Sum of pips always
/// 
/// dice with same face * number of duplicates (summed over all groups)
/// 
/// straights of pip numbers * 2 x length of straight (summed over all straights)
#[derive(Debug)]
pub struct Bang {
    /// The results of the die rolls.
    /// The first 5 are the players result.
    /// The second 5 are the opponent's result.
    pub result: [u8; 10],
    /// Fixed outputs, if 0, the output has not been fixed yet.
    pub fixed_outputs: [u8; 10],
    /// The outcomes from dice removed 
    /// contains up to 5 values for each option.
    pub removed_outputs: [[u8; 5]; 10],

    /// Common Game Data.
    pub base: GameCommonData,
    /// The chances of each point value, for calculating entropy and chances.
    pub chances: BTreeMap<usize, f64>,
    /// How many possibilies there wer overall.
    pub possibilities: f64,
    /// The Current state of the game.
    pub state: BangState,
}

impl Bang {
    /// Create Bang game.
    pub fn new() -> Self {
        Self {
            result: [1; 10],
            fixed_outputs: [0; 10],
            removed_outputs: [[0; 5]; 10],
            base: GameCommonData::new(String::from("Bang"), 
                10.0, 
                1000.0, 
                2.0, 
                Duration::from_secs(30)),
            chances: BTreeMap::new(),
            possibilities: 6.0_f64.powf(5.0),
            state: BangState::HoldingScreen,
        }
    }

    /// # Load chances
    /// 
    /// Loads the spread of chances into memory from the file.
    pub fn load_chances(&mut self) {
        let path = Path::new(BANG_CHANCES);
        let file = fs::read_to_string(path)
            .expect("Could not read file.");
        let splits: Vec<&str> = file.split('\n').collect::<Vec<&str>>();
        self.possibilities = splits[0].parse::<f64>().unwrap();
        println!("{}", self.possibilities);
        for idx in 1..splits.len() {
            let line_vals = splits.get(idx).unwrap().split(":").collect::<Vec<&str>>();
            let key = line_vals[0].parse::<usize>().unwrap();
            let value = line_vals[1].parse::<f64>().unwrap();
            self.chances.insert(key, value);
        }
    }

    /// # Scoring
    /// 
    /// Scoring is simple.
    /// 
    /// Count up all pips.
    /// Multiply base pips by the number of like groups.
    /// - 2 Pair = x2
    /// - 3 pair = x3
    /// - 4 pair = x5
    /// - 5 pair = x10
    /// Multiply by longest straight lengths
    /// - 3 length = x3
    /// - 4 length = x5
    /// - 5 length = x8
    pub fn scoring(&self) -> (usize, usize) {
        let mut p = [0; 5];
        let mut o = [0; 5];
        let mut presults = [0; 6];
        let mut oresults = [0; 6];
        let mut ppoints = 0;
        let mut opoints = 0;

        for idx in 0..5 {
            let presult = self.result[idx] as usize;
            p[idx] = presult;
            ppoints += presult;
            *presults.get_mut(presult-1).unwrap() = *presults.get(presult-1).unwrap() + 1;

            let oresult = self.result[idx+5] as usize;
            o[idx] = oresult;
            opoints += oresult;
            *oresults.get_mut(oresult-1).unwrap() = *oresults.get(oresult-1).unwrap() + 1;
        }

        // groups
        for &side in presults.iter() {
            ppoints = if side == 2 {
                2
            } else if side == 3 {
                3
            } else if side == 4 {
                5
            } else if side == 5 {
                10
            } else { 1 } * ppoints;
        }
        for &side in oresults.iter() {
            opoints = if side == 2 {
                2
            } else if side == 3 {
                3
            } else if side == 4 {
                5
            } else if side == 5 {
                10
            } else { 1 } * opoints;
        }
        // straights
        let mut plongest = 0;
        let mut pcurrent = 0;
        let mut olongest = 0;
        let mut ocurrent = 0;
        for &side in presults.iter() {
            if side > 0 {
                pcurrent += 1;
            } else {
                pcurrent = 0;
            }
            plongest = plongest.max(pcurrent);
        }
        if plongest == 3 {
            ppoints *= 3;
        } else if plongest == 4 {
            ppoints *= 5;
        } else if plongest == 5 {
            ppoints *= 8;
        }
        for &side in oresults.iter() {
            if side > 0 {
                ocurrent += 1;
            } else {
                ocurrent = 0;
            }
            olongest = olongest.max(ocurrent);
        }
        if olongest == 3 {
            opoints *= 3;
        } else if olongest == 4 {
            opoints *= 5;
        } else if olongest == 5 {
            opoints *= 8;
        }

        (ppoints, opoints)
    }
    
    /// # Bet Time Remaining
    /// 
    /// The time remaining for a coin toss bet.
    /// 
    /// For Coin Toss the bet duration is 30 seconds.
    /// 
    /// If no ongoing bet, or the bet time has run out, return 0.0.
    pub fn bet_time_remaining(&self) -> f64 {
        self.base.bet_time_remaining(Duration::from_secs(30)).unwrap_or(0.0)
    }

    /// # Reset
    /// 
    /// Resets fixed_outputs and removed_outputs.
    pub fn reset_fixes(&mut self) {
        self.fixed_outputs = [0; 10];
        self.removed_outputs = [[0; 5]; 10];
    }

    /// # Fix Output
    /// 
    /// Fixes an output for a die. Does not take into account if the die has
    /// already been set.
    pub fn fix_output(&mut self, die: usize, val: u8) {
        self.fixed_outputs[die] = val;
    }

    /// # Remove Option
    /// 
    /// Removes an option from a die.
    /// 
    /// Returns OK, if removed option was added.
    /// Returns Err, if removed option was already added.
    pub fn remove_option(&mut self, die: usize, val: u8) -> Result<(), ()> {
        if self.removed_outputs[die].contains(&val) {
            // if already contained.
            return Err(());
        }
        if self.removed_outputs[die].iter().all(|x| *x != 0) {
            // If all slots taken up.
            return Err(());
        }
        let mut face = 0;
        loop {
            // if not contained, add and replace first 0.
            if self.removed_outputs[die][face] == 0 {
                self.removed_outputs[die][face] = val;
                break;
            }
            face += 1;
        }
        Ok(())
    }

    /// # Roll Dice
    /// 
    /// Rolls dice for us, getting new values for our dice.
    /// 
    /// Sets fixed outputs and rerolls excluded outcomes.
    pub fn roll_dice<R: Rng>(&mut self, rng: &mut R) {
        // get random values.
        self.result = rng.random();
        // reduce to die size and check for restrictions
        for idx in 0..self.result.len() {
            // check if result selected.
            if self.fixed_outputs[idx] != 0 {
                self.result[idx] = self.fixed_outputs[idx];
                // if fixed output, skip to next.
                continue;
            } else { // if output not fixed, reduce to die size.
                // reduce to die size.
                self.result[idx] = self.result[idx] % 6 + 1;
            }
            loop { // reroll if excluded.
                if self.removed_outputs[idx].contains(&self.result[idx]) {
                    self.result[idx] = rng.random::<u8>() % 6 + 1;
                } else {
                    break;
                }
            }
        }
        // Reduce to the size of a dice.
        // for die in &mut self.result {
        //     *die = *die % 6 + 1;
        // }
    }

    /// # Bet
    /// 
    /// Starts a bet, this includes rolling the dice, and setting the current bet
    /// timeout.
    /// 
    /// TODO: Add in potential failure Correction.
    pub fn bet<R: Rng>(&mut self, rng: &mut R) {
        self.roll_dice(rng);
        self.base.bet_start = Some(Instant::now());
    }

    /// # Entropy Gained
    /// 
    /// The entropy gained by the current state.
    /// 
    /// This is based how likely victory or loss was for the given roll.
    pub fn entropy_gained(&self) -> f64 {
        // get the chances of victory or failure based on the player result relative 
        // to the opponent.
        let (player, opponent) = self.scoring();
        // find where the player landed in our probability chart.
        let mut below = 0.0;
        let mut above = 0.0;
        let mut reached = false;
        for (&score, &odds) in self.chances.iter() {
            if reached {
                above += odds;
            }  else {
                if player == score {
                    reached = true;
                    above += odds;
                } else {
                    below += odds;
                }
            }
        }
        // entropy is thus measured as the likelyhood 
        if player > opponent { // if player one, how likely was that win.
            (self.possibilities / below).log2()
        } else { // if lost, how likely was that loss.
            (self.possibilities / above).log2()
        }
    }

    pub fn get_rolls(&self) -> String {
        let mut ret = format!("Opponent's Roll: {},{},{},{},{}\n\n", 
            self.result[5], self.result[6], self.result[7], self.result[8], 
            self.result[9]);
        ret += format!("Your Roll: {},{},{},{},{}\n\n",
            self.result[0], self.result[1], self.result[2], self.result[3], 
            self.result[4]).as_str();
        ret
    }

}

#[derive(Debug)]
pub enum BangState {
    HoldingScreen,
    StartBet,
    InBet,
}