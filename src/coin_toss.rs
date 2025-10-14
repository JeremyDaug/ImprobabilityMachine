use std::time::{Duration, Instant};

use rand::prelude::*;

use crate::game::GameCommonData;

/// # Coin Toss
/// 
/// Coin toss game is about flipping a coin to win a bet.
#[derive(Debug)]
pub struct CoinToss {
    ///  The number of coins being flipped in a group.
    pub heads_chance: f64,
    /// The current result of the game. 
    pub result: bool,
    /// The current state of the game.
    pub state: CoinTossState,
    /// Shared common Data
    pub base: GameCommonData,
}

#[derive(Debug)]
pub enum CoinTossState {
    /// Hold Bet, no active bet ongoing. If show results of previous bet if any.
    Hold,
    /// Start Bet button clicked, turn bet on and enter betting phase.
    /// This includes animating the coin flip. Lasts maybe half a second.
    /// 
    /// Can be skipped.
    StartBet,
    /// Bet is currently active, timer is started. Exits on timeout
    /// complete, player ends it early, or a kickout is triggered.
    InBet,
    /// Bet has been closed out, payout entropy and money to the player.
    ClosingBet,
}

impl CoinToss {
    /// Create a new CoinToss Game
    pub fn new() -> Self {
        Self { 
            heads_chance: 0.5,
            result: true,
            state: CoinTossState::Hold,
            base: GameCommonData::new("Coin Toss".to_string(), 1.0, 100.0, 
                2.0, Duration::from_secs(30))
        }
    }

    /// # Game Loop
    /// 
    /// The loop for the game. Does everything it needs, updating data, checking bets, 
    /// and resolving changes.
    /// 
    /// This should have parameters for the game state, changes/clicks made, and similar stuff.
    pub fn game_loop(&mut self, _interface_state: Option<()>, now: Instant) {
        // Check if we're kicked out currently.
    }

    /// # Start Bet
    /// 
    /// Starts a bet, this includes both flipping the initial coin, and setting the 
    /// current bet timeout.
    /// 
    /// TODO: Add in potential failure correction.
    pub fn bet<R: Rng>(&mut self, rng: &mut R) -> bool {
        self.result = self.flip(rng);
        self.base.bet_start = Some(Instant::now());
        self.result
    }

    /// # Flip
    /// 
    /// Flips coin according tho the heads chance.
    /// 
    /// Returns true if heads, tails is false.
    pub fn flip<R: Rng>(&self, rng: &mut R) -> bool {
        rng.random_bool(self.heads_chance)
    }
}