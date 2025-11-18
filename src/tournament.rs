use std::fs::File;
use std::path::Path;
use std::io::{Write, BufWriter};

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
            writer.write_all(&player.get_number().to_le_bytes())?;
            // write protection?
            for &(opp_num, outcome) in &player.get_matches()[..self.round_number as usize] {
                writer.write_all(&opp_num.to_le_bytes())?;
                writer.write_all(&[outcome as u8])?;
            }
            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }
}
