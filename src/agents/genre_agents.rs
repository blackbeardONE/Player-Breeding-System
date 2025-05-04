use log::info;

pub struct GenreAgent {
    wealth_threshold: f64,
    drop_rate_increase: f64,
}

impl GenreAgent {
    pub fn new(wealth_threshold: f64, drop_rate_increase: f64) -> Self {
        Self {
            wealth_threshold,
            drop_rate_increase,
        }
    }

    pub fn adjust_drop_rate(&self, player_wealth: f64) -> f64 {
        if player_wealth < self.wealth_threshold {
            let new_rate = 1.0 + self.drop_rate_increase;
            info!("GenreAgent: Increasing drop rate by {} due to low wealth {}", self.drop_rate_increase, player_wealth);
            new_rate
        } else {
            1.0
        }
    }
}
