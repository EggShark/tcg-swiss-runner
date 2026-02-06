mod update;

use iced::keyboard::{Event as KEvent, Modifiers};
use iced::widget::operation::{focus_next, focus_previous};
use iced::{keyboard, Length, Subscription, Task};
use iced::widget::{button, column, row, text, text_input};
use tournament_core::swiss::{Outcome, Pairing};
use tournament_core::{player::Player, tournament::Tournament};

fn main() {
    println!("Hello World!");
    let _ = iced::application(TournamentApp::new, TournamentApp::update, TournamentApp::view)
        .subscription(TournamentApp::subscription)
        .run();
}

#[derive(Default)]
pub(crate) struct TournamentApp {
    active_tab: Tabs,
    state: TournamentState,
    tournament: Tournament,
    input_player_name: String,
    input_player_id: String,
    input_player_error: String,
}

impl TournamentApp {
    fn new() -> Self {
        Self::default()
    }

    fn view(&self) -> iced::Element<'_, TournamentEvent> {
        column![
            row![
                button("Matches").on_press(TournamentEvent::MatchesTab),
                button("Players").on_press(TournamentEvent::PlayersTab),
                button("OtherStuff").on_press(TournamentEvent::OtherStuffTab),
            ],
            match self.active_tab {
                Tabs::Matches => self.matches_tab(),
                Tabs::Players => self.player_tab_view(),
                Tabs::OtherStuff => "Other Stuff!".into(),
            },
            "I am top",
        ].into()
    }

    fn subscription(&self) -> Subscription<TournamentEvent> {
        keyboard::listen().map(|e| {
            match e {
                KEvent::KeyPressed {
                    key: keyboard::Key::Named(keyboard::key::Named::Tab),
                    modifiers,
                    ..
                } => if modifiers.contains(Modifiers::SHIFT) {
                    TournamentEvent::ShiftTabPress
                } else {
                    TournamentEvent::TabPress
                }
                _ => TournamentEvent::NonSense
            }
        })
    }

    fn player_tab_view(&self) -> iced::Element<'_, TournamentEvent> {
        // grid of Players
        column![
            (!self.tournament.get_players().is_empty())
                .then(|| row![
                    text("Name").width(Length::FillPortion(1)),
                    text("Id").width(Length::FillPortion(1)),
                    text("W-L-T").width(Length::FillPortion(1)),
                ]),
            column(self.tournament.get_players().iter().map(|p| player_view(p))),
            row![
                text_input("Player Name", &self.input_player_name).on_input(TournamentEvent::PlayerNameUpdate),
                text_input("player_id", &self.input_player_id).on_input(TournamentEvent::PlayerIdUpdate),
            ],
            (!self.input_player_error.is_empty()).then(|| text(&self.input_player_error)),
            button("Add Player").on_press(TournamentEvent::AddPlayer),
        ].into()
    }
    
    fn matches_tab(&self) -> iced::Element<'_, TournamentEvent> {
        column![
            button("Start Tournament").on_press(TournamentEvent::MoveTournamentAlong(TournamentState::DuringRound)),
            column(
                self
                    .tournament
                    .get_pairings()
                    .chunks(2)
                    .enumerate()
                    .map(|(idx, c)| match c {
                        [a, b] => row![pairing_display(a, idx * 2), pairing_display(b, idx*2 + 1)].into(),
                        [a] => row![pairing_display(a, idx*2), row![].width(Length::FillPortion(1))].into(),
                        _ => unreachable!()
                    })
            ),
        ].into()     
    }
}

fn player_view(player: &Player) -> iced::Element<'_, TournamentEvent> {
    row![
        text(player.get_name()).width(Length::FillPortion(1)),
        text(player.get_number()).width(Length::FillPortion(1)),
        {
            let (wins, ties, losses) =  player.get_record();
            text(format!("{}-{}-{}", wins, ties, losses)).width(Length::FillPortion(1))
        },
    ].into()
}

fn pairing_display(pairing: &Pairing, match_number: usize) -> iced::Element<'_, TournamentEvent> {
    let (p1, p2) = pairing.get_players();
    column![
        row![
            text(p1.get_name()).width(Length::FillPortion(1)),
            button("Winner").on_press(TournamentEvent::DeclareMatch(match_number, Outcome::Win)).width(Length::FillPortion(1))
        ],
        match p2 {
            Some(p) => row![
                text(p.get_name()).width(Length::FillPortion(1)),
                button("Winner").on_press(TournamentEvent::DeclareMatch(match_number, Outcome::Loss)).width(Length::FillPortion(1))
            ],
            None => row![text("bye")]
        }
    ].padding(20)
    .into()
}

#[derive(Clone)]
pub(crate) enum TournamentEvent {
    MatchesTab,
    PlayersTab,
    OtherStuffTab,
    PlayerNameUpdate(String),
    PlayerIdUpdate(String),
    MoveTournamentAlong(TournamentState),
    /// used to declare winners where usize is the match number
    DeclareMatch(usize, Outcome),
    AddPlayer,
    TabPress,
    ShiftTabPress,
    NonSense
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tabs {
    #[default]
    Matches,
    Players,
    OtherStuff,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TournamentState {
    #[default]
    PreTournament,
    DuringRound,
    BetweenRounds,
    Fin,
}
