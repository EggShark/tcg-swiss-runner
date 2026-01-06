pub mod player;
pub mod swiss;
pub mod tournament;

pub const DEFUALT_SCORING: swiss::ScoreConfig = swiss::ScoreConfig {
    win: 3,
    tie: 1,
    loss: 0,
};


