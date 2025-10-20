/// # Machine
/// 
/// The Machine is the Improbability machine you are working for.
#[derive(Debug)]
pub struct Machine {
    /// The current level of the building. 
    /// 
    /// Currently, this just converts money to entropy cap at a 2->1 ratio.
    pub level: f64,
    // Extra level/efficiency boosters would go here.
    // Efficiency would be for rerolls, elimination, and selection. 
}

impl Machine {
    pub fn new(level: f64) -> Self {
        Self { 
            level
        }
    }

    /// The maximum Entropy that can be stored by the Improbability Machine.
    /// 
    /// Entropy Cap is equal to 100 + Level.
    pub fn entropy_cap(&self) -> f64 {
        self.level + 100.0
    }
}