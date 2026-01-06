use std::collections::{HashMap, hash_map::Entry};

use rand::seq::SliceRandom;

use crate::player::Player;

const BYE_PLAYER_NUMBER: u16 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
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

#[derive(Debug, PartialEq)]
pub struct Pairing {
    p1: Player,
    p2: Option<Player>,
    winner: Option<Outcome>
}

impl Pairing {
    pub const fn new(p1: Player, p2: Option<Player>) -> Self {
        Self {
            p1,
            p2,
            winner: None,
        }
    }

    pub fn give_outcome(&mut self, outcome: Outcome) {
        self.winner = Some(outcome);
    }

    pub fn is_delcared(&self) -> bool {
        self.winner.is_some()
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
        } else {
            self.p1.add_opponent(BYE_PLAYER_NUMBER, self.winner.unwrap());
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

    fn get_players(&self) -> (&Player, Option<&Player>) {
        (&self.p1, self.p2.as_ref())
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

#[cfg(test)]
mod tests {
    use super::*;

    const SCORES: ScoreConfig = ScoreConfig{
        win: 3,
        tie: 1,
        loss: 0,
    };

    fn generate_players(number: u16) -> Vec<Player> {
        (1..number+1).map(|num| Player::new(num.to_string(), num)).collect()
    }

    #[test]
    fn four_player_all_tie() {
       let mut players = generate_players(4);

       let mut matches = generate_pairings(&mut players, SCORES);
       for pair in &mut matches {
           pair.give_outcome(Outcome::Tie);
       }
       
       let mut players = matches.into_iter()
           .flat_map(|e| {
               let (p1, p2) = e.extract_players();
               [Some(p1), p2]
           })
       .flatten()
       .collect::<Vec<Player>>();

       assert!(players.iter().all(|p| p.extract_record()==(0,0,1)));

       matches = generate_pairings(&mut players, SCORES);
       assert_eq!(2, matches.len());
    }

    #[test]
    fn four_players_one_down_pair() {
        let mut players = generate_players(4);
        let mut matches = generate_pairings(&mut players, SCORES);

        matches[0].give_outcome(Outcome::Win);
        matches[1].give_outcome(Outcome::Tie);

        let mut players = matches.into_iter()
           .flat_map(|e| {
               let (p1, p2) = e.extract_players();
               [Some(p1), p2]
           })
       .flatten()
       .collect::<Vec<Player>>();

       matches = generate_pairings(&mut players, SCORES);
       let (p1, p2) = matches[0].get_players();
       assert_eq!(p1.extract_record(), (1,0,0));
       assert_eq!(p2.unwrap().extract_record(), (0,0,1));

       let (p1, p2) = matches[1].get_players();
       assert_eq!(p1.extract_record(), (0,0,1));
       assert_eq!(p2.unwrap().extract_record(), (0,1,0));
    }

    #[test]
    #[should_panic]
    fn two_players_round_2() {
        let mut players = generate_players(2);
        let mut matches = generate_pairings(&mut players, SCORES);
        matches[0].give_outcome(Outcome::Win);

        let mut players = matches.into_iter()
           .flat_map(|e| {
               let (p1, p2) = e.extract_players();
               [Some(p1), p2]
           })
       .flatten()
       .collect::<Vec<Player>>();

       let _matches = generate_pairings(&mut players, SCORES);
    }


    #[test]
    fn stress_test() {
        let mut players = generate_players(64);
        for _ in 0..12 {
            let mut matches = generate_pairings(&mut players, SCORES);
            for m in &mut matches[1..] {
                m.give_outcome(Outcome::Win);
            }

            // throw some chaos in there !
            matches[0].give_outcome(Outcome::Tie);

            players = matches.into_iter()
                .flat_map(|e| {
                    let (p1, p2) = e.extract_players();
                    [Some(p1), p2]
                })
            .flatten()
            .collect::<Vec<Player>>();
        }
    }

    #[test]
    fn not_test() {
        assert_eq!(!Outcome::Win, Outcome::Loss);
        assert_eq!(!Outcome::Loss, Outcome::Win);
        assert_eq!(!Outcome::Tie, Outcome::Tie)
    }
}
