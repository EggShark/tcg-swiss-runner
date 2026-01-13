use iced::widget::{button, column, container, row};
use tournament_core::player::Player;

fn main() {
    println!("Hello World!");
    let _ = iced::run(TournamentApp::update, TournamentApp::view);
}

type Message = TournamentEvent;

#[derive(Default)]
struct TournamentApp {
    active_tab: Tabs,
    players: Vec<Player>
}

impl TournamentApp {
    fn update(&mut self, message: Message) {
        match message {
            TournamentEvent::MatchesTab => self.active_tab = Tabs::Matches,
            TournamentEvent::PlayersTab => self.active_tab = Tabs::Players,
            TournamentEvent::OtherStuffTab => self.active_tab = Tabs::OtherStuff,
            _ => println!("unhandled :3"),
        } 
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            row![
                button("Matches").on_press(TournamentEvent::MatchesTab),
                button("Players").on_press(TournamentEvent::PlayersTab),
                button("OtherStuff").on_press(TournamentEvent::OtherStuffTab),
            ],
            match self.active_tab {
                Tabs::Matches => "Matches Tab!".into(),
                Tabs::Players => self.player_tab_view(),
                Tabs::OtherStuff => "Other Stuff!".into(),
            },
            "I am top",
        ].into()
    }

    fn player_tab_view(&self) -> iced::Element<'_, Message> {
        // grid of Players
        column![
            column(self.players.iter().map(|p| p.get_name().into())),
            button("Add Player"),
        ].into()
    }
}

#[derive(Clone)]
enum TournamentEvent {
    MatchesTab,
    PlayersTab,
    OtherStuffTab,
    NonSense
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tabs {
    #[default]
    Matches,
    Players,
    OtherStuff,
}
