use std::time::{Duration, Instant};

use macroquad::miniquad::gl::WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB;

/// The current defacto bet duration. Currently set to 30 seconds.
const BET_DURATION: Duration = Duration::from_secs(30);

/// # Game
/// 
/// The data common to all games.
/// 
/// Includes common mechanics between them that can be abstracted away easily.
#[derive(Debug)]
pub struct GameCommonData {
    /// The name of the game.
    pub name: String,

    /// The minimum bet needed to play the game. 
    /// 
    /// Must be >= 0.
    pub bet_min: f64,
    /// The maximum bet that can be used.
    /// 
    /// Can be +inf, must be greater than Min.
    pub bet_max: f64,
    /// Sets the payout rate of the game. You multiply the bet by this value times the 
    /// suspicion modifier to get the final payout.
    pub base_payout: f64,

    /// The current bet locked in. Should be between Min and Max bet, but may be at 0.0, if 
    pub current_bet: f64,

    /// The start time of the current active bet.
    /// 
    /// If no bet active, it is None.
    pub bet_start: Option<Instant>,

    /// The expected win rate of the game up to this point.
    /// 
    /// This is not fixed, but updated based on the bets as they are made.
    pub expected_wins: f64,
    /// The actual win rate the player has recieved.
    pub real_wins: f64,
    /// The gains which are expected by the game up to this point.
    /// 
    /// This is not fixed, but updated based on the bets as they are made.
    pub expected_gains: f64,
    /// The real gains which were achieved by the player.
    pub real_gains: f64,
    /// The current suspicion level of the opponents.
    /// 
    /// Bounded between 0.0 and 1.0.
    /// 
    /// The higher suspicion is, the lower the returns on bets are and
    /// if it gets too high, they may kick you out from the game
    /// 
    /// Being kicked out requires either waiting out a timer or paying them back most
    /// of your earnings.
    /// 
    /// ## Suspicion Brackets
    /// 
    /// [0, 0.25) : No effect
    /// [0.25, 0.5) : Payout reduced by X to a minimum of 1.1.
    /// [0.5, 0.75) : Payout reduced by 2X to a minimum of 1.1.
    /// [0.75, 1.0] : Reduces payout by 4X to a minimum of 1.1 and creates
    ///  (Suspicion - 0.75) * 4 % chance of you being kicked out Auto kicked at 100%.
    pub suspicion: f64,

    /// How long the maximum timeout can last. This is reduced by the suspicion at time 
    /// of being kicked out down to 1/2 of the max duration.
    pub kickout_length_max: Duration,
    /// How much time is left for the current kick out. (updated during kickout update)
    pub kickout_remaining: Duration,
    /// The time the player was kicked out of the game, used to determine when they will be 
    /// allowed back in.
    pub kickout_start_time: Option<Instant>,

    /// When kicked out ,this is used to define how much they want back to let you back in.
    /// 
    /// The buyback value should equal something like:
    /// 
    /// (real_gains - expected_gains) * kickout_buyout_factor * percent_time_remaining.
    pub buyout_factor: f64,
    /// The current Kickout buyout price.
    pub current_kickout_buyout: f64,
}

impl GameCommonData {
    /// # New
    /// 
    /// Simple new to skip over the obviously empty parts at the start.
    pub fn new(name: String, bet_min: f64, bet_max: f64, base_payout: f64, timeout_length_max: Duration) -> Self {
        Self {
            name,
            bet_min,
            bet_max,
            base_payout,
            current_bet: bet_min,
            bet_start: None,
            expected_wins: 0.0,
            real_wins: 0.0,
            expected_gains: 0.0,
            real_gains: 0.0,
            suspicion: 0.0,
            kickout_length_max: timeout_length_max,
            kickout_start_time: None,
            buyout_factor: 0.0,
            kickout_remaining: Duration::ZERO,
            current_kickout_buyout: 0.0,
        }
    }

    /// # Kickout Time Remaining
    /// 
    /// Returns how much time is left until the kickout ends.
    pub fn kickout_time_remaining(&self) -> f64 {
        if let Some(start) = self.kickout_start_time {
            let end = start + self.kickout_length_max;
            end - Instant::now()
        } else {
            Duration::ZERO
        }.as_secs_f64()
    }

    /// # Kickout End Time
    /// 
    /// When the kickout will end.
    /// 
    /// If not currently kicked out, it returns None.
    pub fn kickout_end_time(&self) -> Option<Instant> {
        if let Some(start) = self.kickout_start_time {
            Some(start + self.kickout_length_max)
        } else {
            None
        }
    }

    /// # Kickout Check and Update
    /// 
    /// Checks if the player is still kicked out at this time (Now).
    /// 
    /// If not kicked out it just returns false.
    /// 
    /// If kicked out, but past end time, it returns false and removes our start time.
    /// 
    /// If kicked out and before the end time it returns true (for kicked out).
    /// 
    /// ## Note
    /// 
    /// It's advised to only run this once per frame/update.
    /// 
    /// This does get rid of the kickout start time, but does not reset any other 
    /// kickout data.
    pub fn kickout_update(&mut self, now: Instant) -> bool {
        if let Some(end) = self.kickout_end_time() {
            // if we are currently kicked out.
            if end <= now { // check if we're past the end
                // if so, clear out kickout start time and set remaining to zero.
                self.kickout_start_time = None;
                // update kickout_remaining
                self.kickout_remaining = Duration::ZERO;
                // zero out buyout.
                self.current_kickout_buyout = 0.0;
                true
            } else {
                // Update the kickout remaining based on current time.
                self.kickout_remaining = end - now;
                self.current_kickout_buyout = self.calculate_buyout(now);
                false
            }
        } else { // if not currently kicked out, then we don't need to do anything else.
            false
        }
    }

    /// # Reset Kickout
    /// 
    /// Resets kickout data. That means Suspicion, Expected and real Wins, and expected 
    /// and real gains.
    pub fn reset_kickout(&mut self) {
        self.suspicion = 0.0;
        self.expected_gains = 0.0;
        self.real_gains = 0.0;
        self.expected_wins = 0.0;
        self.real_wins = 0.0;
    }
    
    /// # Calculate Kicout Buyout
    /// 
    /// Calculates the current buyout for the game if it is locked out.
    /// 
    /// The factor is equal to the difference between real and expected gains doubled 
    /// and multiplied further by the buyout factor.
    /// 
    /// Buyouts are rounded to 5 second increments.
    fn calculate_buyout(&self, now: Instant) -> f64 {
        let buyout_max = self.buyout_factor * (self.real_gains - self.expected_gains) * 2.0;
        let time_remaining_factor = (self.kickout_time_remaining() / 
            self.kickout_length_max.as_secs_f64() / 5.0).ceil();
        buyout_max * time_remaining_factor
    }

    /// # Bet Time Remaining
    /// 
    /// How much time remains in the bet, based on current instant.
    /// 
    /// Returns in f64 form instead of Duration, if it returns none, then the bet is over.
    pub fn bet_time_remaining(&self, bet_duration: Duration) -> Option<f64> {
        if let Some(end) = self.bet_end_time(bet_duration) {
            let now = Instant::now();
            if now < end {
                Some((end - now).as_secs_f64())
            } else {
                None
            }
        } else { None }
    }

    pub fn bet_end_time(&self, bet_duration: Duration) -> Option<Instant> {
        if let Some(start) = self.bet_start {
            Some(start + bet_duration)
        } else {
            None
        }
    }
}

/// # Entropy
/// 
/// Calculates the entropy of a given P(robability) value.
/// 
/// # Note
/// 
/// p must be between 0.0 and 1.0 inclusive.
pub fn entropy(p: f64) -> f64 {
    debug_assert!(0.0 < p && p <= 1.0, "P must be betwene 0.0 (not inclusive) and 1.0 (inclusive)");
    -(p).log2()
}

// Entropy Sum value: Not sure if I'd need it, but it's equal to the sum of all 
// entropy(p) values for all possible states. This should be unique per game.