use std::collections::{HashMap, hash_map::Entry};

use rand::seq::SliceRandom;

use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loss,
    Tie
}

impl std::ops::Not for Outcome {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
            Self::Tie => Self::Tie,
        }
    }
}

#[derive(Debug)]
pub struct Pairing {
    p1: Player,
    p2: Option<Player>,
    winner: Option<Outcome>
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
        self.winner = Some(Outcome::Win);
    }

    pub fn p2_wins(&mut self) {
        self.winner = Some(Outcome::Loss);
    }

    pub fn extract_players(mut self) -> (Player, Option<Player>) {
        if self.winner.is_none() {
            return (self.p1, self.p2);
        }

        self.p1.mark_result(self.winner.unwrap());
        if let Some(p2) = &mut self.p2 {
            self.p1.add_opponent(p2.get_number(), self.winner.unwrap());
            p2.mark_result(!self.winner.unwrap());
            p2.add_opponent(self.p1.get_number(), !self.winner.unwrap());
        }

        (self.p1, self.p2)
    }

    pub fn pretty_print(&self) {
        let (p1_w, p1_l, p1_t) = self.p1.extract_record();
        print!("{} ({}/{}/{})", self.p1.get_name(), p1_w, p1_l, p1_t);

        if let Some(p2) = &self.p2 {
            let (p2_w, p2_l, p2_t) = p2.extract_record();
            println!(" vs {}, ({}/{}/{})", p2.get_name(), p2_w, p2_l, p2_t);
        } else {
            println!("Got a Bye")
        }
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
        let e = map.entry(match_points).or_default();
        e.push(player)
    }
    
    let mut left_overs: [Option<Player>; 2] = [None, None];

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
        
        let (left_1, left_2) = (left_overs[0].take(), left_overs[1].take());

        // we have 2 left overs can't play prev oponent
        if let Some(left_over) = left_2 {
            let p2 = players.pop().unwrap();
            pairings.push(Pairing::new(left_over, Some(p2)));
        }

        if let Some(left_over) = left_1 {
            players.push(left_over);
        }

        while players.len() > 1 {
            let p1 = players.pop().unwrap();
            let p2 = players.pop().unwrap();
            if let Some((p1_prev, _)) = p1.get_last_opponent() {
                if p1_prev == p2.get_number() && players.is_empty() {
                    left_overs[0] = Some(p1);
                    left_overs[1] = Some(p2);
                    break;
                } else if p1_prev == p2.get_number() && !players.is_empty() {
                    let p3 = players.pop().unwrap();
                    pairings.push(Pairing::new(p1, Some(p3)));
                    players.push(p2);
                    continue;
                }
            }
            pairings.push(Pairing::new(p1, Some(p2)));
        }

        if !players.is_empty() {
            left_overs[0] = players.pop();
        }
    }

    // will happen in the case that only 2 players on round 3
    assert_eq!(left_overs[1], None);

    // assign bye
    if let Some(left_over) = left_overs[0].take() {
        pairings.push(Pairing::new(left_over, None));
    }

    pairings
}

