use std::{collections::BTreeMap, fs, ops::IndexMut, path::Path, time::Duration};

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
    /// Common Game Data.
    pub base: GameCommonData,
    /// The chances of each point value, for calculating entropy and chances.
    pub chances: BTreeMap<usize, f64>,
    /// How many possibilies there wer overall.
    pub possibilities: f64,
}

impl Bang {
    /// Create Bang game.
    pub fn new() -> Self {
        let mut ret = Self {
            result: [1; 10],
            base: GameCommonData::new(String::from("Bang"), 
                10.0, 
                1000.0, 
                2.0, 
                Duration::from_secs(30)),
            chances: BTreeMap::new(),
            possibilities: 6.0_f64.powf(5.0)
        };
        ret.load_chances();
        ret
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
            self.chances.insert(key, value).unwrap();
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

    /// # Entropy Gained
    /// 
    /// The entropy gained by the current state.
    /// 
    /// This is based how likely victory was for the given roll.
    pub fn entropy_gained(&self) -> f64 {
        0.0
    }
}