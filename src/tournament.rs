use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::error::Error;


use crate::swiss::{generate_pairings, Outcome};
use crate::SCORING;
use crate::{player::Player, swiss::Pairing};

pub struct Tournament {
    round_number: u16,
    players: Vec<Player>,
    pairings: Vec<Pairing>,
    name: String,
}

impl Tournament {
    pub fn new(name: String, players: Vec<Player>) -> Self {
        Self {
            round_number: 0,
            players,
            pairings: Vec::new(),
            name,
        }
    }

    pub fn start_round(&mut self) -> Result<(), TournamentError> {
        if self.players.is_empty() {
            return Err(TournamentError::RoundAlreadyStarted);
        }

        self.pairings = generate_pairings(&mut self.players, SCORING);

        Ok(())
    }

    pub fn report_match(&mut self, match_idx: usize, outcome: Outcome) -> Result<(), TournamentError> {
        if self.pairings.is_empty() {
            return Err(TournamentError::RoundNotImprogress);
        }

        if match_idx >= self.pairings.len() {
            return Err(TournamentError::InvalidMatchIndex(match_idx));
        }

        self.pairings[match_idx].give_outcome(outcome);

        Ok(())
    }

    pub fn finilze_round(&mut self) -> Result<(), TournamentError> {
        if !self.pairings.iter().all(|p| p.is_delcared()) {
            return Err(TournamentError::GamesNotFinished);
        }

        self.players = self.pairings
            .drain(..)
            .flat_map(|e| {
                let (p1, p2) = e.extract_players();
                [Some(p1), p2]
            })
            .flatten()
            .collect::<Vec<Player>>();


        Ok(())
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, out_file: P) -> std::io::Result<()>{
        let file = File::options()
            .write(true)
            // do this last? get everything into a vector and 1 big write?
            // does wipe data consider better option?
            .truncate(true)
            .create(true)
            .open(out_file)?;

        let mut writer = BufWriter::new(file);

        writer.write_all(self.name.as_bytes())?;

        
        writer.write_all(b"\n")?;
        writer.write_all(&self.round_number.to_le_bytes())?;
        writer.write_all(&(self.players.len() as u16).to_le_bytes())?;
        // write out players
        for player in &self.players {
            writer.write_all(player.get_name().as_bytes())?;
            writer.write_all(b"\n")?;
            writer.write_all(&player.get_number().to_le_bytes())?;
            // write protection?
            for &(opp_num, outcome) in &player.get_matches()[..self.round_number as usize] {
                writer.write_all(&opp_num.to_le_bytes())?;
                writer.write_all(&[outcome as u8])?;
            }
        }

        writer.flush()?;

        Ok(())
    }

    pub fn read_from_file<P: AsRef<Path>>(in_file: P) -> Result<Self, TournamentIOError> {
        let file = File::options().read(true).open(in_file)?;
        let mut reader = BufReader::new(file);
        
        let mut name = String::new();
        reader.read_line(&mut name)?;
        
        let mut round_number = [0_u8; 2];
        reader.read_exact(&mut round_number)?;
        let round_number = u16::from_le_bytes(round_number);
        
        let mut number_of_players = [0_u8; 2];
        reader.read_exact(&mut number_of_players)?;
        let number_of_players = u16::from_le_bytes(number_of_players);

        let mut players: Vec<Player> = Vec::new();
        for _ in 0..number_of_players {
            let mut player_name = String::new();
            reader.read_line(&mut player_name)?;

            let mut player_number = [0_u8; 2];
            reader.read_exact(&mut player_number)?;
            let player_number = u16::from_le_bytes(player_number);

            let mut matches = Vec::new();
            let mut wins = 0;
            let mut ties = 0;
            let mut losses = 0;

            // something if player has more details, terminal value?
            // add more data checks
            for _ in 0..round_number {
                let mut opp_number = [0_u8; 2];
                reader.read_exact(&mut opp_number)?;
                let mut outcome = [0_u8];
                reader.read_exact(&mut outcome)?;
                
                let opp_number = u16::from_le_bytes(opp_number);
                let outcome = match outcome[0] {
                    0 => {
                        wins += 1;
                        Outcome::Win
                    },
                    1 => {
                        losses += 1;
                        Outcome::Loss
                    },
                    2 => {
                        ties += 1;
                        Outcome::Tie
                    },
                    e => return Err(TournamentIOError::InvalidResultFound(e)),
                };

                matches.push((opp_number, outcome));
            }
            players.push(Player::from_information(player_name, player_number, (wins,losses,ties), matches));
        }

        Ok(Self {
            name,
            round_number,
            players,
            pairings: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub enum TournamentError {
    RoundAlreadyStarted,
    RoundNotImprogress,
    InvalidMatchIndex(usize),
    GamesNotFinished,
}

impl Display for TournamentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundAlreadyStarted => write!(f, "Attempt to start round when round has already been started"),
            Self::RoundNotImprogress => write!(f, "Attempted to do an opperation that needs a round in progress"),
            Self::InvalidMatchIndex(idx) => write!(f, "Given index of {} is out of bounds", idx),
            Self::GamesNotFinished => write!(f, "Attempted to end tournament with rounds still in progress"),
        }
    }
}

impl Error for TournamentError {}

#[derive(Debug)]
pub enum TournamentIOError {
    Io(std::io::Error),
    MissingNewLineSeperator(usize),
    PlayerHasTooManyRounds(u16, u16),
    InvalidResultFound(u8),
}

impl From<std::io::Error> for TournamentIOError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for TournamentIOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::MissingNewLineSeperator(pos) => write!(f, "expected newline at byte position: {}", pos),
            Self::PlayerHasTooManyRounds(expected, found) => write!(f, "player has played {} rounds expected {}", found, expected),
            Self::InvalidResultFound(err_res) => write!(f, "found {} in result value should be 0,1,2", err_res),
        }
    }
}

impl Error for TournamentIOError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_players(number: u16) -> Vec<Player> {
        (1..number+1).map(|num| Player::new(num.to_string(), num)).collect()
    }

    #[test]
    fn two_plus_two() {
        assert_eq!(2+2,4);
    }
}
