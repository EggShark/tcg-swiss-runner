use player::Player;
use swiss::Outcome;

use crate::swiss::{generate_pairings, ScoreConfig};

mod player;
mod swiss;
mod tournament;

pub const SCORING: ScoreConfig = ScoreConfig {
    win: 3,
    tie: 1,
    loss: 0,
};

fn main() {
    let p1 = Player::new("Bob".to_string(), 1);
    let p2 = Player::new("Alice".to_string(), 2);
    let p3 = Player::new("Carol".to_string(), 3);
    let p4 = Player::new("Carlos".to_string(), 4);

    let mut players = vec![p1, p2, p3, p4];

    let mut parings = generate_pairings(&mut players, SCORING);
    
    for p_match in &mut parings {
        p_match.give_outcome(Outcome::Win);
    }

    let mut players = parings.into_iter()
        .flat_map(|e| {
            let (p1, p2) = e.extract_players();
            [Some(p1), p2]
        }).flatten()
        .collect::<Vec<Player>>();


    println!("Generating round 2 pairings");
    let mut r2 = generate_pairings(&mut players, SCORING);
    for p in r2 {
        p.pretty_print();
    }

}   
