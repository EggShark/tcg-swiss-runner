use crate::swiss::{ScoreConfig, Outcome}; 

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
    wins: u8,
    losses: u8,
    ties: u8,
    player_number: u16,
    opponents: Vec<(u16, Outcome)>,
}

impl Player {
    pub fn new(name: String, player_number: u16) -> Self {
        Self {
            name,
            wins: 0,
            losses: 0,
            ties: 0,
            player_number,
            opponents: Vec::new(),
        }
    }

    pub fn from_information(name: String, player_number: u16, score: (u8,u8,u8), opponents: Vec<(u16, Outcome)>) -> Self {
        Self {
            name,
            player_number,
            wins: score.0,
            losses: score.1,
            ties: score.2,
            opponents,
        }
    }

    pub fn calculate_winrate(&self) -> f32 {
        self.wins as f32 / (self.wins + self.losses) as f32
    }

    pub fn extract_record(&self) -> (u8, u8, u8) {
        (self.wins, self.losses, self.ties)
    }

    pub fn caluculate_match_points(&self, score_config: ScoreConfig) -> u8 {
        (self.wins * score_config.win) + (self.losses * score_config.loss) + (self.ties * score_config.tie)
    }

    pub fn get_number(&self) -> u16 {
        self.player_number
    }

    pub fn get_last_opponent(&self) -> Option<(u16, Outcome)> {
        self.opponents.last().copied()
    }

    pub fn get_matches(&self) -> &[(u16, Outcome)] {
        &self.opponents
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_opponent(&mut self, op_number: u16, outcome: Outcome) {
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Loss => self.losses += 1,
            Outcome::Tie => self.ties += 1,
        }
        self.opponents.push((op_number, outcome));
    }

    /// (wins, losses, ties)
    pub fn get_record(&self) -> (u8, u8, u8) {
        (self.wins, self.losses, self.ties)
    }
}
