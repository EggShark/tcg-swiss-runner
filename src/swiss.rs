use std::collections::{HashMap, hash_map::Entry};

use rand::seq::SliceRandom;

use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Result {
    Win,
    Loss,
    Tie
}

#[derive(Debug)]
pub struct Pairing {
    p1: Player,
    p2: Option<Player>,
    winner: Option<Result>
}

impl Pairing {
    pub fn new(p1: Player, p2: Option<Player>) -> Self {
        Self {
            p1,
            p2,
            winner: None,
        }
    }

    pub fn p1_wins(&mut self) {
        self.winner = Some(Result::Win);
    }

    pub fn p2_wins(&mut self) {
        self.winner = Some(Result::Loss);
    }

    pub fn extract_players(mut self) -> (Player, Option<Player>) {
        match self.winner {
            Some(Result::Win) => {
                self.p1.mark_win();
                if let Some(p2) = &mut self.p2 {
                    p2.mark_win();
                }
            },
            Some(Result::Loss) => {
                if let Some(p2) = &mut self.p2 {
                    self.p1.mark_loss();
                    p2.mark_win();
                }
            },
            Some(Result::Tie) => {
                self.p1.mark_tie();
                if let Some(p2) = &mut self.p2 {
                    p2.mark_tie();
                }
            },
            None => {},
        };

        (self.p1, self.p2)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScoreConfig {
    pub win: u8,
    pub loss: u8,
    pub tie: u8,
}

pub fn generate_pairings(players: &mut Vec<Player>, scoring: ScoreConfig) -> Vec<Pairing> {
    let mut map: HashMap<u8, Vec<Player>> = HashMap::new();
    let mut pairings = Vec::new();
    let mut max_mp = 0;

    while let Some(player) = players.pop() {
        let match_points = player.caluculate_match_points(scoring);
        max_mp = std::cmp::max(match_points, max_mp);
        let e = map.entry(max_mp).or_default();
        e.push(player)
    }
    
    let mut left_over = None;

    loop {
        let mut players = match map.remove(&max_mp) {
            Some(e) => e,
            None => {
                if max_mp == 0 {
                    break;
                } else {
                    max_mp -= 1;
                    continue;
                }
            },
        };
        
        let mut rng = rand::rng();
        players.shuffle(&mut rng);
        
        if let Some(downer) = left_over {
            
        }

        while players.len() > 1 {
            let p1 = players.pop().unwrap();
            let p2 = players.pop().unwrap();
            pairings.push(Pairing::new(p1, Some(p2)));
        }
        left_over = players.pop();
    }
   
    pairings
}

