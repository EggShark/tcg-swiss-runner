use iced::widget::{button, column, row};

fn main() {
    println!("Hello World!");
    let _ = iced::run(TournamentApp::update, TournamentApp::view);
}

type Message = TournamentEvent;

#[derive(Default)]
struct TournamentApp {
    tab: Tabs,
}

impl TournamentApp {
    fn update(&mut self, message: Message) {
        match message {
            TournamentEvent::MatchesTab => self.tab = Tabs::Matches,
            TournamentEvent::PlayersTab => self.tab = Tabs::Players,
            TournamentEvent::OtherStuffTab => self.tab = Tabs::OtherStuff,
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
            match self.tab {
                Tabs::Matches => "Matches Tab!",
                Tabs::Players => "Player Tab!",
                Tabs::OtherStuff => "Other Stuff!",
            },
            "I am top",
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
