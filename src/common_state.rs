/// # Common State
/// 
/// The state of the game, including resources, game being looked at, buttons
/// pressed, and similar stuff.
/// 
/// This is what get's passed around 
pub struct CommonState {
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