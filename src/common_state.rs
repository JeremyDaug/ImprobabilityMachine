use std::time::{Duration, Instant};

use crate::machine::machine::Machine;

/// # Common State
/// 
/// The state of the game, including resources, game being looked at, buttons
/// pressed, and similar stuff.
/// 
/// This is what get's passed around 
pub struct CommonState {
    /// mTh ename of the player
    pub player_name: String,
    /// The money owned by the player. If it ever goes below CoinToss.base.bet_min, 
    /// then it's game over.
    /// 
    /// This is jokingly measured in £sd (Pounds, shillings, pence) for the player,
    /// but is measured directly here.
    pub money: f64,
    /// The Entropy the player has available to them. This is measured in bits and
    /// is spent manipulating the games in question.
    pub entropy: f64,

    /// Which game is currently active. This is just a value to define which of the 
    /// 10-20 games are currently active and being played.
    /// 
    /// This may be altered to an enum or some other thing, but for now, it'll probably 
    /// just be a simple value range.
    pub active_game: u8,

    /// The current bet selected for the game.
    /// 
    /// Should be between the current active game's min and max as well as less than
    /// or equal to the current money available.
    pub current_bet: f64,

    /// Click on the start bet Button.
    pub button_clicked: ButtonAction,

    /// The Entropy Machine of the player. Sets the cap on how much entropy can be stored
    /// in one moment.
    pub machine: Machine,

    /// The last time the save was made since game start.
    pub last_prior_save: Instant,
    /// How long the game has been going on.
    /// 
    /// Added to and updated periodically.
    pub game_length: Duration,
}

impl CommonState {
    pub fn empty() -> Self {
        Self {
            player_name: String::new(),
            money: 0.0,
            entropy: 0.0,
            active_game: 0,
            current_bet: 0.0,
            button_clicked: ButtonAction::None,
            machine: Machine {
                level: 0.0
            },
            last_prior_save: Instant::now(),
            game_length: Duration::ZERO,
        }
    }

    pub fn new(player_name: String) -> Self {
        Self {
            player_name,
            money: 240.0,
            entropy: 100.0,
            active_game: 0,
            current_bet: 1.0,
            button_clicked: ButtonAction::None,
            machine: Machine { level: 0.0 },
            last_prior_save: Instant::now(),
            game_length: Duration::ZERO
        }
    }

    pub fn add_entropy(&mut self, entropy_gained: f64) {
        self.entropy += entropy_gained;
        self.entropy = self.entropy.min(self.machine.entropy_cap());
    }
    
    /// # Save Str(ing)
    /// 
    /// Creates a string which is coverted into a byte array, and returned.
    /// 
    /// Should be a standard layout. Very simple, very dumb shit.
    /// 
    /// I should be ashamed of myself, but I'm not. 
    /// 
    /// If anyone wishes to complain, they can send their complaint to me by
    /// throwing it in the trash.
    pub fn save_str(&mut self) -> String {
        // update game length and last prior save
        self.game_length += Instant::now() - self.last_prior_save;
        self.last_prior_save = Instant::now();
        let mut output = String::new();
        output += format!("{},", self.player_name).as_str();
        output += format!("{},", self.money).as_str();
        output += format!("{},", self.entropy).as_str();
        output += format!("{},", self.machine.level).as_str();
        output += format!("{},", self.game_length.as_secs_f64()).as_str();
        output
    }
    
    /// # Load State
    /// 
    /// Loads the common state data from the file.
    pub fn load_state(&mut self, file: String) {
        let splits: Vec<&str> = file.split(',').collect::<Vec<&str>>();
        self.player_name = splits.get(0).unwrap().to_string();
        self.money = splits[1].parse::<f64>().unwrap();
        self.entropy = splits[2].parse::<f64>().unwrap();
        self.machine.level = splits[3].parse::<f64>().unwrap();
        self.game_length = Duration::from_secs_f64(splits[4].parse::<f64>().unwrap());
        self.last_prior_save = Instant::now();
    }
}

pub enum GameState {
    MainMenu,
    SaveScreen,
    LoadScreen,
    GameScreen
}

pub enum ButtonAction {
    /// Placeholder, no button being clicked right now.
    None,
    /// Starts the bet, only one bet may be started at a time.
    StartBet,
    /// Button to end the bet, closing out the bet timer.
    EndBet,
    /// Button to buy out the kickout Timer.
    Buyout,
}