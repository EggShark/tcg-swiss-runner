use iced::{Length, widget::{button, column, row, text, text_input}};
use tournament_core::{player::Player, tournament::Tournament};

fn main() {
    println!("Hello World!");
    let _ = iced::run(TournamentApp::update, TournamentApp::view);
}

type Message = TournamentEvent;

#[derive(Default)]
struct TournamentApp {
    active_tab: Tabs,
    tournament: Tournament,
    input_player_name: String,
    input_player_id: String,
    input_player_error: String,
}

impl TournamentApp {
    fn update(&mut self, message: Message) {
        match message {
            TournamentEvent::MatchesTab => self.active_tab = Tabs::Matches,
            TournamentEvent::PlayersTab => self.active_tab = Tabs::Players,
            TournamentEvent::OtherStuffTab => self.active_tab = Tabs::OtherStuff,
            TournamentEvent::PlayerIdUpdate(v) => self.input_player_id = v,
            TournamentEvent::PlayerNameUpdate(v) => self.input_player_name = v,
            TournamentEvent::AddPlayer => self.add_player(),
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

    fn add_player(&mut self) {
        let player_id = self.input_player_id.parse::<u16>();
        match player_id {
            Ok(_) => {},
            Err(_e) => {
                self.input_player_error = "Invalid Player ID entered".to_string(); //TODO make more
                                                                                   //descripitive
                return;
            }
        }
        let mut player_name = String::new();
        std::mem::swap(&mut player_name, &mut self.input_player_name);
        self.input_player_id.clear();
        let player = Player::new(player_name, player_id.unwrap());
        self.tournament.add_player(player);
        self.input_player_error.clear();
    }
}

fn player_view(player: &Player) -> iced::Element<'_, Message> {
    row![
        text(player.get_name()).width(Length::FillPortion(1)),
        text(player.get_number()).width(Length::FillPortion(1)),
        {
            let (wins, ties, losses) =  player.get_record();
            text(format!("{}-{}-{}", wins, ties, losses)).width(Length::FillPortion(1))
        },
    ].into()
}

#[derive(Clone)]
enum TournamentEvent {
    MatchesTab,
    PlayersTab,
    OtherStuffTab,
    PlayerNameUpdate(String),
    PlayerIdUpdate(String),
    AddPlayer,
    NonSense
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tabs {
    #[default]
    Matches,
    Players,
    OtherStuff,
}
