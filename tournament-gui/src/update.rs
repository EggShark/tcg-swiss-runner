use crate::{Tabs, TournamentEvent, TournamentApp, TournamentState};

use iced::Task;
use iced::widget::operation::{focus_next, focus_previous};

use tournament_core::player::Player;
use tournament_core::swiss::Outcome;

impl TournamentApp {
    pub(crate) fn update(&mut self, message: TournamentEvent) -> Task<TournamentEvent> {
        let mut final_task = Task::none();
        match message {
            TournamentEvent::MatchesTab => self.active_tab = Tabs::Matches,
            TournamentEvent::PlayersTab => self.active_tab = Tabs::Players,
            TournamentEvent::OtherStuffTab => self.active_tab = Tabs::OtherStuff,
            TournamentEvent::PlayerIdUpdate(v) => self.input_player_id = v,
            TournamentEvent::PlayerNameUpdate(v) => self.input_player_name = v,
            TournamentEvent::AddPlayer => self.add_player(),
            TournamentEvent::DeclareMatch(idx, res) => self.tournament.report_match(idx, res).unwrap(),
            TournamentEvent::OpenMatchDialoge(idx) => self.dialog_state = Some(crate::DialogStates::MatchReportState { match_index: idx }),
            TournamentEvent::TabPress => final_task = focus_next(),
            TournamentEvent::ShiftTabPress => final_task = focus_previous(),
            TournamentEvent::MoveTournamentAlong(TournamentState::DuringRound) => {
                self.tournament.start_round().unwrap();
            }
            TournamentEvent::NonSense => {},
            _ => println!("unhandled :3"),
        }

        final_task
    }
    
    fn add_player(&mut self) {
        let player_id = self.input_player_id.parse::<u16>();
        let player_id = match player_id {
            Ok(id) => id,
            Err(_) => {
                self.input_player_error = "Player ID must be a number".to_string();
                return;
            }
        };

        if let Some(id) = self.tournament.get_players().iter().map(|p| p.get_number()).find(|&id| id == player_id) {
            self.input_player_error = format!("Player ID of {} is not unique", id);
            return;
        }

        let mut player_name = String::new();
        std::mem::swap(&mut player_name, &mut self.input_player_name);
        self.input_player_id.clear();
        let player = Player::new(player_name, player_id);
        self.tournament.add_player(player);
        self.input_player_error.clear();
    }
}
